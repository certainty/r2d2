//! The engine is the main abstraction that you use
//! to interact with the key-value store system.
//!
//! The engine owns all key-value pairs and takes care of
//! interacting with them, delegating storage to its subsystem.
//!
//! It maintains the local state as well as providing
//! the necessary data transfer to update the cluster state if required.
//!
//! It presents itself with a dictionary-like interfacer where each operation
//! might fail. This is deliberate since every operation has to potentially interact
//! with the OS or the network which are unreliable components.
use std::ops::Deref;

pub mod default;
pub mod storage;

// A key is an arbitray sequence of bytes
// For concenience there are conversions from String and to Vec<u8>
#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct Key {
    pub data: Vec<u8>,
}

impl Key {
    pub fn new(data: Vec<u8>) -> Key {
        Key { data }
    }

    pub fn from_string(s: &str) -> Key {
        Key {
            data: s.as_bytes().to_vec(),
        }
    }
}

impl Deref for Key {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct Value {
    pub data: Vec<u8>,
}

impl Deref for Value {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Value {
    pub fn new(data: Vec<u8>) -> Value {
        Value { data }
    }

    pub fn from_string(s: &str) -> Value {
        Value {
            data: s.as_bytes().to_vec(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Error {
    StorageError(storage::lsm::Error),
}

pub trait Engine {
    // Insert a key value pair into the store
    //
    // when this function returns successfully, the following guarantees hold:
    // * the change is durable on the local node.
    // * a local lookup will return the inserted value (unless there was an update inbetween)
    fn set(&mut self, key: Key, value: Value) -> Result<Option<Value>, Error>;

    // Delete a key from the store
    //
    // The key doesn not need to exist in which case the operation is a noop.
    // It is expected that the operation returns the value of the key that has
    // been deleted if it existed.
    //
    // If the function returns successfully, the following guarantees hold:
    // * the change is durable on the local node.
    // * the key/value can not be found anymore (unless it has been re-inserted)
    fn del(&mut self, key: &Key) -> Result<Option<Value>, Error>;

    // Lookup a value for the given key
    //
    // Find a value for the given key if it exists.
    // This operation might fail, e.g. when implementatons need to access the
    // filesystem or the network.
    fn get(&self, key: &Key) -> Result<Option<Value>, Error>;

    // List all the currently stored keys
    //
    // This is purely for debug reasons as in any real system the amount of keys
    // might grow way too large to return them all in a vector.
    fn keys(&self) -> Result<Vec<Key>, Error>;
}
