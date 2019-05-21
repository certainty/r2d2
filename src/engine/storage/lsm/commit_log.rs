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
use std::convert::From;

#[derive(Debug, PartialEq)]
pub enum Error {
    SerializationError,
    IoError(String),
    LockError
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

impl From<std::boxed::Box<bincode::ErrorKind>> for Error {
  fn from(_e: std::boxed::Box<bincode::ErrorKind>) -> Self {
    Error::SerializationError
  }
}

#[derive(Serialize, Deserialize, Debug)]
enum Operation<'a> {
    Set(& 'a [u8], & 'a [u8]),
    Delete(& 'a [u8]),
}

pub struct CommitLog {
    writer: Arc<Mutex<File>>
}

impl CommitLog {
    pub fn new(storage_directory: &path::Path) -> Result<CommitLog, io::Error> {
        let file_path = storage_directory.join(path::Path::new("commit.log"));
        let log_file  = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .append(true)
                        .open(&file_path)?;

        trace!("opened commit_log at {:?}", file_path);
        Ok(CommitLog{ writer: Arc::new(Mutex::new(log_file)) })
    }

    pub fn commit_set(&mut self, k: &[u8], v: &[u8]) -> Result<(), Error> {
        self.write_operation(&Operation::Set(k, v))
    }

    pub fn commit_delete(&mut self, k: &[u8]) -> Result<(), Error> {
        self.write_operation(&Operation::Delete(k))
    }

    fn write_operation(&mut self, operation: &Operation) -> Result<(), Error> {
      let data = bincode::serialize(operation)?;
      self.writer.lock()?.write(&data)?;
      Ok(())
    }
}

