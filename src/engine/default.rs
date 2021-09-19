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
use crate::engine::EngineIterator;
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

    fn get(&self, key: &Key) -> Result<Option<&Value>> {
        trace!(target: "engine", "Lookup {:?}", key);

        Ok(self.lsm.get(&key)?)
    }

    fn iter(&self) -> EngineIterator {
        self.lsm.iter()
    }
}
