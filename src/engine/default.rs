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
use super::Result;
use super::{Engine, Key, Value};
use std::fmt::Debug;

pub struct DefaultEngine {
    lsm: lsm::LSM,
}

pub fn new(storage_directory: PathBuf) -> DefaultEngine {
    let storage_path = fs::canonicalize(&storage_directory).unwrap();
    let lsm = lsm::init(storage_path.as_path()).unwrap();

    DefaultEngine { lsm }
}

impl Engine for DefaultEngine {
    fn set<K: Into<Key> + Debug, V: Into<Value> + Debug>(
        &mut self,
        key: K,
        value: V,
    ) -> Result<Option<Value>> {
        trace!(target: "engine", "Insert {:?} -> {:?}", key, value);

        Ok(self.lsm.set(key.into(), value.into())?)
    }

    fn del(&mut self, key: &Key) -> Result<Option<Value>> {
        trace!(target: "engine", "Delete {:?}", key);

        Ok(self.lsm.del(&key)?)
    }

    fn get(&self, key: &Key) -> Result<Option<Value>> {
        trace!(target: "engine", "Lookup {:?}", key);

        Ok(self.lsm.get(&key)?.cloned())
    }

    // TODO: maybe we should implement an iterator instead to make it more efficient
    fn keys(&self) -> Result<Vec<Key>> {
        trace!(target: "engine", "List keys");
        let byte_keys = self.lsm.keys()?;
        let keys = byte_keys.iter().map(|k| Key::new(k.to_vec())).collect();
        Ok(keys)
    }
}
