use super::Engine;
use log::trace;
use std::collections::BTreeMap;

pub struct DefaultEngine {
  // The choice of a BTreeMap is kind of abritary at the moment,
  // since we don't care too much about the performance of the local
  // store so far.
  dict: BTreeMap<String, String>,
}

pub fn new() -> DefaultEngine {
  DefaultEngine {
    dict: BTreeMap::new(),
  }
}

impl Engine for DefaultEngine {
  fn insert(&mut self, key: String, value: String) -> Result<Option<String>, String> {
    trace!(target: "engine", "Insert {} -> {}", key, value);
    self.dict.insert(key, value);
    Ok(None)
  }

  fn delete(&mut self, key: String) -> Result<Option<String>, String> {
    trace!(target: "engine", "Delete {}", key);
    Ok(self.dict.remove(&key))
  }

  fn lookup(&self, key: String) -> Result<Option<&String>, String> {
    trace!(target: "engine", "Lookup {}", key);
    Ok(self.dict.get(&key))
  }

  fn list_keys(&self) -> Result<Vec<String>, String> {
    trace!(target: "engine", "List keys");
    Ok(self.dict.keys().cloned().collect())
  }
}
