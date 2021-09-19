/// The engine is the main abstraction that you use
/// to interact with the key-value store system.
///
/// The engine owns all key-value pairs and takes care of
/// interacting with them, delegating storage to its subsystem.
///
/// It maintains the local state as well as providing
/// the necessary data transfer to update the cluster state if required.
///
/// It presents itself with a dictionary-like interfacer where each operation
/// might fail. This is deliberate since every operation has to potentially interact
/// with the OS or the network which are unreliable components.
use thiserror::Error;

pub mod default;
pub mod key;
pub mod storage;
pub mod value;

// re-exports for convenience
pub use key::Key;
use std::fmt::Debug;
pub use value::Value;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    StorageError(#[from] storage::lsm::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Engine {
    // Insert a key value pair into the store
    //
    // when this function returns successfully, the following guarantees hold:
    // * the change is durable on the local node.
    // * a local lookup will return the inserted value (unless there was an update in between)
    fn set<K: Into<Key> + Debug, V: Into<Value> + Debug>(
        &mut self,
        key: K,
        value: V,
    ) -> Result<Option<Value>>;

    // Delete a key from the store
    //
    // The key doesn not need to exist in which case the operation is a noop.
    // It is expected that the operation returns the value of the key that has
    // been deleted if it existed.
    //
    // If the function returns successfully, the following guarantees hold:
    // * the change is durable on the local node.
    // * the key/value can not be found anymore (unless it has been re-inserted)
    fn del(&mut self, key: &Key) -> Result<Option<Value>>;

    // Lookup a value for the given key
    //
    // Find a value for the given key if it exists.
    // This operation might fail, e.g. when implementatons need to access the
    // filesystem or the network.
    fn get(&self, key: &Key) -> Result<Option<Value>>;

    // List all the currently stored keys
    //
    // This is purely for debug reasons as in any real system the amount of keys
    // might grow way too large to return them all in a vector.
    fn keys(&self) -> Result<Vec<Key>>;
}
