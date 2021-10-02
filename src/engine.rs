/// The engine is the main abstraction to interact with the key-value store system.
///
/// The engine owns all key-value pairs and takes care of
/// interacting with them, delegating storage to its subsystem.
///
/// It maintains the local state as well as providing
/// the necessary data transfer to update the cluster state if required.
///
/// It presents itself with a dictionary-like interface where each operation
/// might fail. This is deliberate since every operation has to potentially interact
/// with the OS or the network which are unreliable components.
// re-exports for convenience
pub use key::Key;
use log;
use std::fmt::Debug;
use thiserror::Error;
pub use value::Value;

use crate::engine::configuration::Configuration;

pub mod configuration;
pub mod directories;
pub mod key;
pub mod storage;
pub mod value;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ConfigurationError(#[from] configuration::Error),
    #[error(transparent)]
    StorageConfigurationError(#[from] storage::lsm::configuration::Error),

    #[error(transparent)]
    StorageError(#[from] storage::lsm::Error),
    #[error(transparent)]
    FileSystemError(#[from] directories::Error),
}

/// The engine encapsulates local storage, replication and distribution
pub struct Engine {
    lsm: storage::lsm::LSM,
}

impl Engine {
    /// Starts up the engine and makes sure it's operational.
    pub fn start(config: Configuration) -> Result<Self> {
        directories::setup_directories(&config)?;

        let lsm = storage::lsm::LSM::new(config.storage)?;
        Ok(Self { lsm })
    }

    /// Insert a key value pair into the store
    ///
    /// when this function returns successfully, the following guarantees hold:
    /// * the change is durable on the local node.
    /// * a local lookup will return the inserted value (unless there was an update in between)
    pub fn set<K: Into<Key> + Debug, V: Into<Value> + Debug>(
        &mut self,
        key: K,
        value: V,
    ) -> Result<Option<Value>> {
        log::trace!(target: "engine", "Insert {:?} -> {:?}", key, value);
        Ok(self.lsm.set(key.into(), value.into())?)
    }

    /// Delete a key from the store
    ///
    /// The key does not need to exist in which case the operation is a noop.
    /// It is expected that the operation returns the value of the key that has
    /// been deleted if it existed.
    ///
    /// If the function returns successfully, the following guarantees hold:
    /// * the change is durable on the local node.
    /// * the key/value can not be found anymore (unless it has been re-inserted)
    pub fn del(&mut self, key: &Key) -> Result<Option<Value>> {
        log::trace!(target: "engine", "Delete {:?}", key);
        Ok(self.lsm.del(&key)?)
    }

    /// Lookup a value for the given key
    ///
    /// Find a value for the given key if it exists.
    /// This operation might fail, e.g. when implementatons need to access the
    /// filesystem or the network.
    pub fn get(&self, key: &Key) -> Result<Option<Value>> {
        log::trace!(target: "engine", "Lookup {:?}", key);
        Ok(self.lsm.get(&key)?)
    }

    pub fn iter(&self) -> EngineIterator {
        self.lsm.iter()
    }
}

pub struct EngineIterator<'a> {
    iter: storage::lsm::Iter<'a>,
}

impl<'a> EngineIterator<'a> {
    pub fn new(iter: storage::lsm::Iter<'a>) -> Self {
        Self { iter }
    }
}

impl<'a> Iterator for EngineIterator<'a> {
    type Item = (&'a Key, &'a Value);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
