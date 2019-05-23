//! Simple append only commit log
//!
//! This module provides the funcitonality for a simple append only commit-log.
//! Every write is first written to this log before any further action is taken.
//! In case of a crash the commitlog can be used to reconstruct the state prior to the
//! crash.
//! Note that the log writes to the filesystem without flushing, thus leaving
//! the ultimate control over when the write happens to the OS at the benefit of a faster
//! write through the FS cache.

extern crate crc;
use std::convert::From;
use std::fs;
use std::io;
use std::path;
use std::sync::{Arc, Mutex};

use serde;
use serde::{Deserialize, Serialize};

use byteorder::LittleEndian as Endianess;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{BufReader, BufWriter, Read};

const VERSION: u8 = 1;
const STANZA: &str = "r2d2::commitlog";

type Result<T> = std::result::Result<T, Error>;

// public API
pub fn open(path: &path::Path) -> Result<CommitLogReader> {
    CommitLogReader::open(path)
}

pub fn create(path: &path::Path) -> Result<CommitLogWriter> {
    CommitLogWriter::create(path)
}

pub fn resume(path: &path::Path) -> Result<CommitLogWriter> {
    CommitLogWriter::resume(path)
}

#[derive(Debug, PartialEq)]
pub enum Error {
    SerializationError,
    IoError(String),
    LockError,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e.to_string())
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Error::LockError
    }
}

#[derive(Deserialize, Serialize, PartialEq)]
struct FileHeader {
    stanza: Vec<u8>,
    version: u8,
}

impl FileHeader {
    pub fn new(stanza: &str, version: u8) -> FileHeader {
        FileHeader {
            stanza: stanza.as_bytes().to_vec(),
            version,
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum Operation {
    Set(Vec<u8>, Vec<u8>),
    Delete(Vec<u8>),
}

// internal reqresentation of an operation frame
#[derive(Serialize, Deserialize, PartialEq)]
enum OperationFrame<'a> {
    Set(&'a [u8], &'a [u8]),
    Delete(&'a [u8]),
}

pub struct CommitLogWriter {
    file: Arc<Mutex<io::BufWriter<fs::File>>>,
}

impl CommitLogWriter {
    pub fn resume(path: &path::Path) -> Result<CommitLogWriter> {
        let writer = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(path)?;

        Ok(CommitLogWriter {
            file: Arc::new(Mutex::new(BufWriter::new(writer))),
        })
    }

    pub fn create(path: &path::Path) -> Result<CommitLogWriter> {
        let mut writer = fs::OpenOptions::new().create(true).write(true).open(path)?;
        let header = FileHeader::new(STANZA, VERSION);

        write_data(&mut writer, header)?;

        Ok(CommitLogWriter {
            file: Arc::new(Mutex::new(BufWriter::new(writer))),
        })
    }

    pub fn write_set(&mut self, k: &Vec<u8>, v: &Vec<u8>) -> Result<usize> {
        let mut file = self.file.lock()?;
        write_data(&mut *file, OperationFrame::Set(k, v))
    }

    pub fn write_delete(&mut self, k: &Vec<u8>) -> Result<usize> {
        let mut file = self.file.lock()?;
        write_data(&mut *file, OperationFrame::Delete(k))
    }
}

pub struct CommitLogReader {
    header: FileHeader,
    file: io::BufReader<fs::File>,
}

impl CommitLogReader {
    pub fn open(path: &path::Path) -> Result<CommitLogReader> {
        let mut reader = fs::OpenOptions::new().read(true).open(path)?;
        let header: FileHeader = read_data(&mut reader)?;

        Ok(CommitLogReader {
            header: header,
            file: BufReader::new(reader),
        })
    }

    pub fn read(&mut self) -> Result<Operation> {
        read_data(&mut self.file)
    }
}

// utilities
fn write_data<W, D>(w: &mut W, data: D) -> Result<usize>
where
    W: io::Write,
    D: serde::Serialize,
{
    let serialized = bincode::serialize(&data).map_err(|_| Error::SerializationError)?;
    write_frame(w, &serialized)
}

// TODO: make sure we really need the owned variant here
// could be a problem since data may be copied
fn read_data<R, D>(r: &mut R) -> Result<D>
where
    R: io::Read,
    D: serde::de::DeserializeOwned,
{
    let mut buf = Vec::new();
    read_frame(r, &mut buf)?;
    let value = bincode::deserialize(buf.as_slice()).map_err(|_e| Error::SerializationError)?;
    Ok(value)
}

fn read_frame<R>(reader: &mut R, buf: &mut Vec<u8>) -> Result<usize>
where
    R: io::Read,
{
    let size = reader.read_u64::<Endianess>()?;
    reader.take(size).read_to_end(buf)?;
    Ok(size as usize)
}

fn write_frame<W>(writer: &mut W, data: &[u8]) -> Result<usize>
where
    W: io::Write,
{
    writer.write_u64::<Endianess>(data.len() as u64)?;
    writer.write_all(data)?;
    writer.flush()?;
    Ok(data.len())
}
