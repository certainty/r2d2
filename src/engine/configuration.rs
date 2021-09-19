use crate::engine::storage;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub storage: storage::Configuration,
}

impl Configuration {
    pub fn new(storage: storage::Configuration) -> Self {
        Configuration { storage }
    }
}
