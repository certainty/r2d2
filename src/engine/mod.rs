//! The engine is the main abstraction that you use
//! to interact with the key-value store system.  
//!
//! It takes care of maintaining the local state as well as providing
//! the necessary data transfer to update the cluster state if required.
//!
//! It presents itself with a dictionary-like interfacer where each operation
//! might fail. This is deliberate since every operation has to potentially interact
//! with the OS or the network which are unreliable components.
#[derive(Debug, Clone, Ord, Eq, PartialOrd, PartialEq, Hash)]
pub struct Key(Vec<u8>);

impl Key {
  pub fn from_string(str: String) -> Key {
    Key(str.into_bytes())
  }

  // inefficient implementation to construct a key
  // intended to be used in tests
  pub fn from_str(str: &str) -> Key {
    Key::from_string(String::from(str))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value(Vec<u8>);

impl Value {
  pub fn from_string(str: String) -> Value {
    Value(str.into_bytes())
  }

  // inefficient implementation to create a value
  // which is intended to be used in tests
  pub fn from_str(str: &str) -> Value {
    Value::from_string(String::from(str))
  }
}

pub trait Engine {
  // Insert a key value pair into the store
  //
  // when this function returns successfully, the following guarantees hold:
  // * the change is durable on the local node.
  // * a local lookup will return the inserted value (unless there was an update inbetween)
  fn insert(&mut self, key: Key, value: Value) -> Result<Option<Value>, String>;

  // Delete a key from the store
  //
  // The key doesn not need to exist in which case the operation is a noop.
  // It is expected that the operation returns the value of the key that has
  // been deleted if it existed.
  //
  // If the function returns successfully, the following guarantees hold:
  // * the change is durable on the local node.
  fn delete(&mut self, key: Key) -> Result<Option<Value>, String>;

  fn lookup(&self, key: Key) -> Result<Option<&Value>, String>;

  // List all the currently stored keys.
  // This is purely for debug reasons as in any real system the amount of keys
  // might grow way too large to return them all in a vector.
  fn list_keys(&self) -> Result<Vec<Key>, String>;
}

pub mod default;
