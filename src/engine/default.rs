//! Default engine implementation
//!
//! This module provides the default implementation for the storage engine.
//! It uses a log structured merge tree architecture for local storage.
//!
//!

use log::trace;
use std::fs;
use std::path::PathBuf;

use super::storage::lsm;
use super::Error as EngineError;
use super::{Engine, Key, Value};

impl From<lsm::Error> for EngineError {
    fn from(e: lsm::Error) -> Self {
        super::Error::StorageError(e)
    }
}

pub struct DefaultEngine {
    lsm: lsm::LSM,
}

pub fn new(storage_directory: PathBuf) -> DefaultEngine {
    let storage_path = fs::canonicalize(&storage_directory).unwrap();
    let lsm = lsm::new(storage_path.as_path()).unwrap();

    DefaultEngine { lsm }
}

impl Engine for DefaultEngine {
    fn set(&mut self, key: Key, value: Value) -> Result<Option<Value>, EngineError> {
        trace!(target: "engine", "Insert {:?} -> {:?}", key, value);
        self.lsm.set(key.data, value.data)?;
        Ok(None)
    }

    fn del(&mut self, key: Key) -> Result<Option<Value>, EngineError> {
        trace!(target: "engine", "Delete {:?}", key);
        let value = self.lsm.del(key.data)?;
        Ok(value.map(|v| Value::new(v)))
    }

    fn get(&self, key: Key) -> Result<Option<Value>, EngineError> {
        trace!(target: "engine", "Lookup {:?}", key);
        let value = self.lsm.get(key.data)?;

        Ok(value.map(|v| Value::new(v.clone())))
    }

    fn keys(&self) -> Result<Vec<Key>, EngineError> {
        trace!(target: "engine", "List keys");
        let byte_keys = self.lsm.keys()?;
        let keys = byte_keys.iter().map(|k| Key::new(k.to_vec())).collect();
        Ok(keys)
    }
}
