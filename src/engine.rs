//! The engine is the main abstraction that you use
//! to interact with the key-value store system.  
//!
//! It takes care of maintaining the local state as well as providing
//! the necessary data transfer to update the cluster state if required.
//!
//! It presents itself with a dictionary-like interfacer where each operation
//! might fail. This is deliberate since every operation has to potentially interact
//! with the OS or the network which are unreliable components.
pub trait Engine {
  // Insert a key value pair into the store
  //
  // when this function returns successfully, the following guarantees hold:
  // * the change is durable on the local node.
  // * a local lookup will return the inserted value (unless there was an update inbetween)
  fn insert(&mut self, key: String, value: String) -> Result<Option<String>, String>;

  // Delete a key from the store
  //
  // The key doesn not need to exist in which case the operation is a noop.
  // It is expected that the operation returns the value of the key that has
  // been deleted if it existed.
  //
  // If the function returns successfully, the following guarantees hold:
  // * the change is durable on the local node.
  fn delete(&mut self, key: String) -> Result<Option<String>, String>;

  fn lookup(&self, key: String) -> Result<Option<&String>, String>;

  // List all the currently stored keys.
  // This is purely for debug reasons as in any real system the amount of keys
  // might grow way too large to return them all in a vector.
  fn list_keys(&self) -> Result<Vec<String>, String>;
}

pub mod default;
