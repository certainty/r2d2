use std::collections::BTreeMap;

use crate::engine::{Key, Value};

/// The memtable is the fast C0 system in the LSM.
/// It has two main properties:
/// 1. fast key based operations (lookup and insertion)
/// 2. sorted iteration over keys (to dump to SSTables)

pub enum Entry {
    Tombstone,
    Val(Value),
}

pub struct BTreeMemtable(BTreeMap<Key, Entry>);
pub type Iter<'a> = std::collections::btree_map::Iter<'a, Key, Entry>;

impl BTreeMemtable {
    pub fn new() -> Self {
        BTreeMemtable(BTreeMap::new())
    }

    pub fn remove(&mut self, key: &Key) -> Option<Entry> {
        self.0.remove(key)
    }

    pub fn insert(&mut self, key: Key, value: Value) -> Option<Entry> {
        self.0.insert(key, Entry::Val(value))
    }

    pub fn get(&self, key: &Key) -> Option<&Value> {
        match self.0.get(key) {
            Some(Entry::Val(value)) => Some(value),
            _ => None,
        }
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn iter(&self) -> Iter {
        self.0.iter()
    }
}
