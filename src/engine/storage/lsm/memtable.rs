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

pub trait Memtable {
    // places a tombstone for `key` and returns the value it had before removal
    fn remove(&mut self, key: &Key) -> Option<Entry>;

    // insert the `value` for the given `key` and return the previous associated value with `key` if there was one.
    fn insert(&mut self, key: Key, value: Value) -> Option<Entry>;

    fn get(&self, key: &Key) -> Option<&Value>;

    // clear the table make sure it's empty
    fn clear(&mut self);

    // return an iterator for all elements in the memtable
    fn iter<'a>(&self) -> dyn Iterator<Item = (&'a Key, &'a Entry)>;
}

#[repr(transparent)]
pub struct BTreeMemtable(BTreeMap<Key, Entry>);

impl BTreeMemtable {
    pub fn new() -> Self {
        BTreeMemtable(BTreeMap::new())
    }
}

impl Memtable for BTreeMemtable {
    fn remove(&mut self, key: &Key) -> Option<Entry> {
        self.0.remove(key)
    }

    fn insert(&mut self, key: Key, value: Value) -> Option<Entry> {
        self.0.insert(key, Entry::Val(value))
    }

    fn get(&self, key: &Key) -> Option<&Value> {
        match self.0.get(key) {
            Some(Entry::Val(value)) => Some(value),
            _ => None,
        }
    }

    fn clear(&mut self) {
        self.0.clear()
    }

    fn iter<'a>(&self) -> dyn Iterator<Item = (&'a Key, &'a Entry)> {
        self.0.iter()
    }
}
