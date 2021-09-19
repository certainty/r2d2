pub mod lsm;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub version: u32,
    pub storage_path: PathBuf,
}

impl Configuration {
    pub fn new(storage_path: PathBuf) -> Self {
        Self {
            storage_path,
            version: 1,
        }
    }
}
