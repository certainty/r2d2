/// Simple append only write ahead log
///
/// This module provides the funcitonality for a simple append only wal.
/// Every write is first written to this log before any further action is taken.
/// In case of a crash the wal can be used to reconstruct the state prior to the
/// crash.
/// **Note** that the log writes to the filesystem without flushing, thus leaving
/// the ultimate control over when the write happens to the OS at the benefit of a faster
/// write through the FS cache.
pub mod reader;
pub mod serialization;
pub mod writer;
extern crate crc;
use super::binary_io as binio;
use crate::engine::storage::lsm::wal::reader::WalReader;
use serde::{self, Deserialize, Serialize};
use std::convert::From;
use std::path;
use thiserror::Error;
use writer::WalWriter;

const WAL_FILE_NAME: &str = "wal.log";

type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    BinIoError(#[from] binio::Error),
    #[error("LockError")]
    LockError,
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Error::LockError
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
/// The operation to persist with the WAL
pub enum Operation<K, V> {
    /// Use this to commit a set operation for the provided key-value pair
    Set(K, V),
    /// Use this to commit the deletion of the provided key
    Delete(K),
}

/// Representation of the Write Ahead Log
pub struct WalManager {
    active_file: path::PathBuf,
}

impl WalManager {
    /// Initialize the WAL directory
    ///
    /// The `init` function creates the required file structures to
    /// allow the WAL to work properly.
    ///
    /// It is safe to call this method multiple times.
    pub fn init(storage_path: &path::Path) -> Result<WalManager> {
        let wal_path = storage_path.join("wal");
        let wal_file_name = wal_path.join(WAL_FILE_NAME);
        std::fs::create_dir_all(&wal_path)?;

        Ok(WalManager {
            active_file: wal_file_name,
        })
    }

    /// Uses the state in WAL directory to determine if a recovery is needed
    pub fn recovery_needed(&self) -> bool {
        self.active_file.exists()
    }

    /// Create a *new* WAL file and returns a `WalWriter`, which
    /// can be used to add records to the WAL.
    ///
    /// If the file already exists it will be *truncated*.
    /// If you don't want that use the `resume` method instead.
    pub fn create(&self) -> Result<WalWriter> {
        WalWriter::create(&self.active_file)
    }

    /// Resume writes to an existing WAL file.
    ///
    /// Contrary to `create` this will open the file in append mode.
    pub fn resume(&self) -> Result<WalWriter> {
        WalWriter::resume(&self.active_file)
    }

    /// Opens an existing WAL for reading
    ///
    /// This is used when recovery is needed and the records need to be played back.
    pub fn open(&self) -> Result<WalReader> {
        WalReader::open(&self.active_file)
    }

    /// A null WAL will accept writes but will never actually write anything.
    /// This can be used to disabled WAL temporarily.
    pub fn null(&self) -> Result<WalWriter> {
        WalWriter::null()
    }
}

#[cfg(test)]
mod tests {
    use super::binio;
    use super::Operation;
    use std::io;

    #[test]
    fn read_your_write() {
        let mut writer = io::Cursor::new(Vec::new());
        let foo = "foo".as_bytes();
        let bar = "bar".as_bytes();

        assert!(binio::write_data(&mut writer, Operation::Set(foo, bar)).is_ok());

        let mut reader = io::Cursor::new(writer.into_inner());

        let op: Operation<Vec<u8>, Vec<u8>> = binio::read_data_owned(&mut reader).unwrap();

        assert_eq!(Operation::Set(foo.to_vec(), bar.to_vec()), op)
    }
}
