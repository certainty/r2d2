//! Simple append only write ahead log
//!
//! This module provides the funcitonality for a simple append only wal.
//! Every write is first written to this log before any further action is taken.
//! In case of a crash the wal can be used to reconstruct the state prior to the
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

use super::binary_io as binio;
use crate::engine::storage::lsm::wal::Error::BinIoError;
use byteorder::LittleEndian;
use byteorder::{ReadBytesExt, WriteBytesExt};
use log::{error, trace};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

const VERSION: u8 = 1;
const STANZA: &str = "r2d2::wal";
const WAL_FILE_NAME: &str = "write_ahead.log";

type Result<T> = std::result::Result<T, Error>;

// public API
pub fn init(storage_path: &path::Path) -> Result<Wal> {
    let wal_path = storage_path.join("wal");
    let wal_file_name = wal_path.join(WAL_FILE_NAME);
    std::fs::create_dir_all(&wal_path)?;

    Ok(Wal {
        directory: wal_path,
        active_file: wal_file_name,
    })
}

pub struct Wal {
    directory: path::PathBuf,
    active_file: path::PathBuf,
}

impl Wal {
    pub fn recovery_needed(&self) -> bool {
        self.active_file.exists()
    }

    pub fn create(&self) -> Result<WalWriter> {
        WalWriter::create(&self.active_file)
    }

    pub fn open(&self) -> Result<WalReader> {
        WalReader::open(&self.active_file)
    }

    pub fn resume(&self) -> Result<WalWriter> {
        WalWriter::resume(&self.active_file)
    }

    pub fn null(&self) -> Result<WalWriter> {
        WalWriter::null()
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    IoError(io::ErrorKind),
    BinIoError(binio::Error),
    LockError,
}

impl From<binio::Error> for Error {
    fn from(e: binio::Error) -> Self {
        BinIoError(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e.kind())
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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Operation<T> {
    Set(T, T),
    Delete(T),
}

pub struct WalWriter {
    file: Arc<Mutex<io::BufWriter<fs::File>>>,
}

impl WalWriter {
    pub fn resume(path: &path::Path) -> Result<WalWriter> {
        let writer = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(path)?;

        Ok(WalWriter {
            file: Arc::new(Mutex::new(BufWriter::new(writer))),
        })
    }

    pub fn null() -> Result<WalWriter> {
        let writer = fs::OpenOptions::new()
            .append(true)
            .write(true)
            .open(path::Path::new("/dev/null"))?;

        Ok(WalWriter {
            file: Arc::new(Mutex::new(BufWriter::new(writer))),
        })
    }

    pub fn create(path: &path::Path) -> Result<WalWriter> {
        let mut writer = fs::OpenOptions::new().create(true).write(true).open(path)?;
        let header = FileHeader::new(STANZA, VERSION);

        binio::write_data(&mut writer, header)?;

        Ok(WalWriter {
            file: Arc::new(Mutex::new(BufWriter::new(writer))),
        })
    }

    pub fn write(&mut self, op: Operation<&[u8]>) -> Result<usize> {
        let mut file = self.file.lock()?;
        let size = binio::write_data(&mut *file, op)?;
        file.flush()?;

        Ok(size)
    }
}

pub struct WalReader {
    header: FileHeader,
    file: io::BufReader<fs::File>,
}

impl WalReader {
    pub fn open(path: &path::Path) -> Result<WalReader> {
        let mut reader = fs::OpenOptions::new().read(true).open(path)?;
        let header: FileHeader = binio::read_data_owned(&mut reader)?;

        trace!(
            target: "WAL",
            "wal successfully opened. version = {}",
            header.version
        );

        Ok(WalReader {
            header,
            file: BufReader::new(reader),
        })
    }

    pub fn read(&mut self) -> Result<Operation<Vec<u8>>> {
        let data = binio::read_data_owned(&mut self.file)?;
        Ok(data)
    }
}

impl Iterator for WalReader {
    type Item = Result<Operation<Vec<u8>>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read() {
            Err(Error::BinIoError(binio::Error::IoError(io::ErrorKind::UnexpectedEof))) => None,
            Err(e) => Some(Err(e)),
            ok => Some(ok),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn read_your_write() {
        let mut writer = io::Cursor::new(Vec::new());
        let foo = "foo".as_bytes();
        let bar = "bar".as_bytes();

        assert!(binio::write_data(&mut writer, Operation::Set(foo, bar)).is_ok());

        let mut reader = io::Cursor::new(writer.into_inner());

        let op: Operation<Vec<u8>> = binio::read_data_owned(&mut reader).unwrap();

        assert_eq!(Operation::Set(foo.to_vec(), bar.to_vec()), op)
    }
}
