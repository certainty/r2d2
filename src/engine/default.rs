use super::{Engine, Key, Value};
use super::storage::lsm::LSM;
use log::trace;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::fs;

pub struct DefaultEngine {
  // The choice of a BTreeMap is kind of abritary at the moment,
  // since we don't care too much about the performance of the local
  // store so far.
  lsm: LSM,
  dict: BTreeMap<Key, Value>,
}

pub fn new(storage_directory: PathBuf) -> DefaultEngine {
  let storage_path = fs::canonicalize(&storage_directory).unwrap();

  DefaultEngine {
    lsm: LSM::new(storage_path),
    dict: BTreeMap::new(),
  }
}

impl<'a> Engine for DefaultEngine {
  fn insert(&mut self, key: Key, value: Value) -> Result<Option<Value>, String> {
    trace!(target: "engine", "Insert {:?} -> {:?}", key, value);
    self.dict.insert(key, value);
    Ok(None)
  }

  fn delete(&mut self, key: Key) -> Result<Option<Value>, String> {
    trace!(target: "engine", "Delete {:?}", key);
    Ok(self.dict.remove(&key))
  }

  fn lookup(&self, key: Key) -> Result<Option<&Value>, String> {
    trace!(target: "engine", "Lookup {:?}", key);
    Ok(self.dict.get(&key))
  }

  fn list_keys(&self) -> Result<Vec<Key>, String> {
    trace!(target: "engine", "List keys");
    Ok(self.dict.keys().cloned().collect())
  }
}
