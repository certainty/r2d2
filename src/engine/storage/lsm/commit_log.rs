//! Simple append only commit log
//!
//! This module provides the funcitonality for a simple append only commit-log.
//! Every write is first written to this log before any further action is taken.
//! In case of a crash the commitlog can be used to reconstruct the state prior to the
//! crash.
//! Note that the log writes to the filesystem without flushing, thus leaving
//! the ultimate control over when the write happens to the OS at the benefit of a faster
//! write through the FS cache.

use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::io;
use std::io::Write;
use std::result::Result;
use std::path;
use std::fs::*;
use log::trace;

pub enum Error {
    SerializationError,
    IoError(io::Error),
    LockError
}

impl std::convert::From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl<T> std::convert::From<std::sync::PoisonError<T>> for Error {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Error::LockError
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Operation {
    Set(Vec<u8>, Vec<u8>),
    Delete(Vec<u8>),
}

pub struct CommitLog {
    path: path::PathBuf,
    writer: Arc<Mutex<File>>
}

impl CommitLog {
    pub fn new(storage_directory: path::PathBuf) -> Result<CommitLog, io::Error> {
        let file_path = storage_directory.join(path::Path::new("commit.log"));
        trace!("commit_log: path is {:?}", file_path);
        let log_file  = OpenOptions::new().append(true).open(&file_path)?;

        Ok(CommitLog{ path: file_path, writer: Arc::new(Mutex::new(log_file)) })
    }

    pub fn commitSet(&mut self, k: &Vec<u8>, v: &Vec<u8>) -> Result<usize, Error> {
        self.writeOperation(&Operation::Set(k.clone(), v.clone()))
    }

    pub fn commitDelete(&mut self, k: &Vec<u8>) -> Result<usize, Error> {
        self.writeOperation(&Operation::Delete(k.clone()))
    }

    fn writeOperation(&mut self, operation: &Operation) -> Result<usize, Error> {
        if let Ok(data) = bincode::serialize(operation) {
            let mut writer = self.writer.lock()?;
            let written    = writer.write(&data)?;
            Ok(written)
        } else {
            Err(Error::SerializationError)
        }
    }
}

