//! The engine is the main abstraction that you use
//! to interact with the key-value store system.
//!
//! It takes care of maintaining the local state as well as providing
//! the necessary data transfer to update the cluster state if required.
//!
//! It presents itself with a dictionary-like interfacer where each operation
//! might fail. This is deliberate since every operation has to potentially interact
//! with the OS or the network which are unreliable components.
//!

pub mod default;
pub mod storage;

#[derive(Debug, Clone, Ord, Eq, PartialOrd, PartialEq, Hash)]
pub struct Key(Vec<u8>);

impl Key {
  pub fn as_bytes(&self) -> &[u8] {
    &self.0
  }
}

impl From<&str> for Key {
  fn from(s: &str) -> Key {
    Key(s.into())
  }
}

impl From<&Key> for String {
  fn from(k: &Key) -> String {
    String::from_utf8(k.0.to_owned()).unwrap()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value(Vec<u8>);

impl Value {
  pub fn as_bytes(&self) -> &[u8] {
    &self.0
  }
}

impl From<&str> for Value {
  fn from(s: &str) -> Value {
    Value(s.into())
  }
}

impl From<&Value> for String {
  fn from(v: &Value) -> String {
    String::from_utf8(v.0.to_owned()).unwrap()
  }
}


pub trait Engine {
  // Insert a key value pair into the store
  //
  // when this function returns successfully, the following guarantees hold:
  // * the change is durable on the local node.
  // * a local lookup will return the inserted value (unless there was an update inbetween)
  fn insert(&mut self, key: impl Into<Key>, value: impl Into<Value>) -> Result<Option<Value>, String>;

  // Delete a key from the store
  //
  // The key doesn not need to exist in which case the operation is a noop.
  // It is expected that the operation returns the value of the key that has
  // been deleted if it existed.
  //
  // If the function returns successfully, the following guarantees hold:
  // * the change is durable on the local node.
  // * the key/value can not be found anymore (unless it has been re-inserted)
  fn delete(&mut self, key: impl Into<Key>) -> Result<Option<Value>, String>;

  //Lookup a value for the given key
  //
  // Find a value for the given key if it exists.
  // This operation might fail, e.g. when implementatons need to access the
  // filesystem or the network.
  fn lookup(&self, key: impl Into<Key>) -> Result<Option<&Value>, String>;

  // List all the currently stored keys
  //
  // This is purely for debug reasons as in any real system the amount of keys
  // might grow way too large to return them all in a vector.
fn list_keys(&self) -> Result<Vec<Key>, String>;
}
