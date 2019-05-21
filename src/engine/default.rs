//! Default engine implementation
//!
//! This module provides the default implementation for the storage engine.
//! It uses a log structured merge tree architecture for local storage.
//!
//!

use log::trace;
use std::path::PathBuf;
use std::fs;

use super::{Engine, Key, Value};
use super::storage::lsm;
use super::Error as EngineError;

impl From<lsm::Error> for EngineError {
  fn from(e: lsm::Error) -> Self {
    super::Error::StorageError(e)
  }
}

pub struct DefaultEngine {
  // The choice of a BTreeMap is kind of abritary at the moment,
  // since we don't care too much about the performance of the local
  // store so far.
  lsm: lsm::LSM
}

pub fn new(storage_directory: PathBuf) -> DefaultEngine {
  let storage_path = fs::canonicalize(&storage_directory).unwrap();

  DefaultEngine {
    lsm: lsm::LSM::new(storage_path.as_path())
  }
}

impl Engine for DefaultEngine {
  fn insert(&mut self, key: Key, value: Value) -> Result<Option<Value>, EngineError> {
    trace!(target: "engine", "Insert {:?} -> {:?}", key, value);
    self.lsm.insert(key.data, value.data)?;
    Ok(None)
  }

  fn delete(&mut self, key: Key) -> Result<Option<Value>, EngineError> {
    trace!(target: "engine", "Delete {:?}", key);
    let value = self.lsm.remove(key.data)?;
    Ok(value.map(|v| Value::new(v)))
  }

  fn lookup(&self, key: Key) -> Result<Option<Value>, EngineError> {
    trace!(target: "engine", "Lookup {:?}", key);
    let value = self.lsm.lookup(key.data)?;

    Ok(value.map(|v| Value::new(v.clone())))
  }

  fn list_keys(&self) -> Result<Vec<Key>, EngineError> {
    trace!(target: "engine", "List keys");
    let byte_keys = self.lsm.keys()?;
    let keys = byte_keys.iter().map(|k| Key::new(k.to_vec())).collect();
    Ok(keys)
  }
}
