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
    lsm: LSM::new(storage_path.as_path()),
    dict: BTreeMap::new(),
  }
}

impl Engine for DefaultEngine {
  fn insert(&mut self, key: impl Into<Key>, value: impl Into<Value>) -> Result<Option<Value>, String> {
    let (k, v) = (key.into(), value.into());
    trace!(target: "engine", "Insert {:?} -> {:?}", k, v);
    self.lsm.insert(k.as_bytes(), v.as_bytes()).unwrap();
    self.dict.insert(k, v);
    Ok(None)
  }

  fn delete(&mut self, key: impl Into<Key>) -> Result<Option<Value>, String> {
    let k = key.into();

    trace!(target: "engine", "Delete {:?}", k);
    Ok(self.dict.remove(&k))
  }

  fn lookup(&self, key: impl Into<Key>) -> Result<Option<&Value>, String> {
    let k = key.into();

    trace!(target: "engine", "Lookup {:?}", k);
    Ok(self.dict.get(&k))
  }

  fn list_keys(&self) -> Result<Vec<Key>, String> {
    trace!(target: "engine", "List keys");
    Ok(self.dict.keys().cloned().collect())
  }
}
