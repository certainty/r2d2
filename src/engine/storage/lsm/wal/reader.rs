use super::binio;
use super::serialization::FileHeader;
/// A WalReader that gives access to committed operations in a convenient manner.
///
/// Use the reader to replay committed operations. It provides an iterator
/// to the underlying `Operation`, which is assumed to be enough to
/// restore state from the WAL.
///
use super::Result;
use crate::engine::storage::lsm::wal::{Error, Operation};
use crate::engine::{Key, Value};
use std::io::BufReader;
use std::{fs, io, path};

pub struct WalReader {
    header: FileHeader,
    file: io::BufReader<fs::File>,
}

impl WalReader {
    pub fn open(path: &path::Path) -> Result<Self> {
        let mut reader = fs::OpenOptions::new().read(true).open(path)?;
        let header: FileHeader = binio::read_data_owned(&mut reader)?;

        log::trace!("wal successfully opened. version = {}", header.version);

        Ok(WalReader {
            header,
            file: BufReader::new(reader),
        })
    }

    /// Reads the next comitted operation from the WAL
    ///
    /// Use this to implement you own logic if you can't use the provided Iterator implementation.
    /// TODO: shouldn't that be borrowed?
    pub fn read(&mut self) -> Result<Operation<Key, Value>> {
        let data = binio::read_data_owned(&mut self.file)?;
        Ok(data)
    }
}

impl Iterator for WalReader {
    type Item = Result<Operation<Key, Value>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read() {
            Err(Error::BinIoError(binio::Error::IoError(io_error)))
                if io_error.kind() == io::ErrorKind::UnexpectedEof =>
            {
                None
            }
            Err(e) => Some(Err(e)),
            ok => Some(ok),
        }
    }
}
