use crate::engine::storage::lsm;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    StorageConfigurationError(#[from] lsm::configuration::Error),
}

#[derive(Debug, Clone)]
pub struct Configuration {
    pub storage: lsm::configuration::Configuration,
}

impl Configuration {
    pub fn new(storage: lsm::configuration::Configuration) -> Self {
        Configuration { storage }
    }
}

pub struct Builder {
    pub storage: lsm::configuration::Builder,
}

impl Builder {
    pub fn build(self) -> Result<Configuration> {
        Ok(Configuration {
            storage: self.storage.build()?,
        })
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            storage: lsm::configuration::Builder::default(),
        }
    }
}
