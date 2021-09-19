use crate::engine::Result;
use crate::engine::{directories, storage};

#[derive(Debug, Clone)]
pub struct Configuration {
    pub storage: storage::Configuration,
}

impl Configuration {
    pub fn default() -> Result<Self> {
        Ok(Self::new(storage::Configuration::default()?))
    }

    pub fn new(storage: storage::Configuration) -> Self {
        Configuration { storage }
    }
}
