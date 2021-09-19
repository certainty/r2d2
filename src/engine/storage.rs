pub mod lsm;
use crate::engine::directories;
use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error)]
pub enum Error {
    #[error(transparent)]
    DirectoriesError(#[from] directories::Error),
}

#[derive(Debug, Clone)]
pub struct Configuration {
    pub version: u32,
    pub storage_path: PathBuf,
}

impl Configuration {
    pub fn default() -> Result<Self> {
        Ok(Self::new(directories::default_storage_path()?, 1))
    }

    pub fn new(storage_path: PathBuf, version: u32) -> Self {
        Self {
            storage_path,
            version,
        }
    }
}
