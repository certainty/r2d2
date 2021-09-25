use std::path::PathBuf;
use thiserror::Error;
use ubyte::{ByteUnit, ToByteUnit};

type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("OutOfBound: the provided configuration is outside the allowed bounds. {0}")]
    OutOfBound(String),
    #[error("InvalidStoragePath: the provided storage path `{1}` is invalid. {0}")]
    InvalidStoragePath(String, PathBuf),
}

#[derive(Debug, Clone)]
pub struct Configuration {
    /// the path to the main directory for the storage engine
    /// all components will use a file structure beneath this base path
    pub storage_path: PathBuf,
    /// memtable size in bytes
    pub max_memtable_size: ByteUnit,
}

pub struct Builder {
    storage_path: Option<PathBuf>,
    max_memtable_size: Option<ByteUnit>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            storage_path: None,
            max_memtable_size: None,
        }
    }

    pub fn build(self) -> Result<Configuration> {
        Ok(Configuration {
            storage_path: self.storage_path.unwrap(),
            max_memtable_size: self.max_memtable_size.unwrap(),
        })
    }

    pub fn with_storage_path<T: Into<PathBuf>>(&mut self, path: T) -> Result<&mut Self> {
        let storage_path = path.into();
        Self::assert_valid_storage_path(&storage_path)?;
        self.storage_path = Some(storage_path);
        Ok(self)
    }

    pub fn with_memtable_size<T: Into<ByteUnit>>(&mut self, size: T) -> Result<&mut Self> {
        self.max_memtable_size = Some(size.into());
        Ok(self)
    }

    fn assert_valid_storage_path(storage_path: &PathBuf) -> Result<()> {
        if !storage_path.is_dir() {
            Err(Error::InvalidStoragePath(
                "The provided storage path is not a directory".into(),
                storage_path.clone(),
            ))
        } else {
            Ok(())
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            storage_path: None,
            max_memtable_size: Some(512.megabytes()),
        }
    }
}
