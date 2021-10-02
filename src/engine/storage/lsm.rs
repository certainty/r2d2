use crate::engine::storage::lsm::sstable::Slab;
use crate::engine::storage::lsm::wal::writer::WalWriter;
/// The LSM implements a log structured merge tree using SSTables as C1
///
/// Docs: https://en.wikipedia.org/wiki/Log-structured_merge-tree/
///
/// The module exposes a struct that can be used to construct
/// an LSM by providing appropriate implementations for the underlying
/// components. The architecture will take care of all aspects
/// related to the management of the local LSM. It might spawn additional
/// threads.
use crate::engine::{EngineIterator, Key, Value};
use configuration::Configuration;
use log;
use std::collections::BTreeMap;
use thiserror::Error;

pub mod binary_io;
pub mod configuration;
pub mod sstable;
pub mod wal;

type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SSTableError(#[from] sstable::Error),
    #[error(transparent)]
    WalError(#[from] wal::Error),
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
}

/// The LSM implementation is comprised of some classical components.
/// It uses a write-ahead-log (WAL) to make operations durable on the local node.
/// It uses an in memory index / table to have a fast C0 system for key-value pairs
/// It uses SSTables in the C1 system to allow relatively fast look-up and very fast
/// (io-optmized) disc access for huge amounts of data.
pub struct LSM {
    config: Configuration,
    wal: WalWriter,
    memtable: Memtable,
    slabs: Vec<Slab>,
}

/// The memtable is the fast C0 system in the LSM.
/// It has two main properties:
/// 1. fast key based operations (lookup and insertion)
/// 2. sorted iteration over keys (to dump to SSTables)
type Memtable = BTreeMap<Key, Value>;
pub type Iter<'a> = std::collections::btree_map::Iter<'a, Key, Value>;

impl LSM {
    pub fn new(config: Configuration) -> Result<LSM> {
        let wal = wal::WalManager::init(&config.storage_path)?;

        let lsm = if wal.recovery_needed() {
            Self::init_with_recovery(config, &wal)
        } else {
            Self::init_clean(config, &wal)
        };

        log::info!(target: "LSM","lsm subsystem initialized and ready");
        lsm
    }

    fn init_clean(config: Configuration, wal: &wal::WalManager) -> Result<LSM> {
        let memtable = Memtable::new();

        log::info!(target: "LSM", "starting lsm with fresh commit log",);

        Ok(LSM {
            config,
            wal: wal.create()?,
            memtable,
            slabs: Vec::new(),
        })
    }

    fn init_with_recovery(config: Configuration, wal: &wal::WalManager) -> Result<LSM> {
        log::info!(target: "LSM", "starting recovery from WAL");

        let memtable = Memtable::new();
        let mut lsm_for_repair = LSM {
            config,
            wal: wal.null()?,
            memtable,
            slabs: Vec::new(),
        };

        Self::recover(&mut lsm_for_repair, &wal)?;
        log::info!(target: "LSM", "recovery completed successfully");

        Ok(LSM {
            wal: wal.resume()?,
            ..lsm_for_repair
        })
    }

    fn recover(lsm: &mut LSM, wal: &wal::WalManager) -> Result<()> {
        let reader = wal.open()?;

        for result_of_op in reader {
            match result_of_op? {
                wal::Operation::Set(key, value) => {
                    lsm.set(key, value)?;
                    ()
                }
                wal::Operation::Delete(key) => {
                    lsm.del(&key)?;
                    ()
                }
            }
        }

        Ok(())
    }

    pub fn set(&mut self, k: Key, v: Value) -> Result<Option<Value>> {
        self.wal.write(wal::Operation::Set(&k, &v))?;
        Ok(self.memtable.insert(k, v))
    }

    pub fn del(&mut self, k: &Key) -> Result<Option<Value>> {
        self.wal.write(wal::Operation::Delete(k))?;
        Ok(self.memtable.remove(k))
    }

    pub fn get(&self, k: &Key) -> Result<Option<Value>> {
        Ok(self.get_c0(k).or(self.get_c1(k)?))
    }

    pub fn iter(&self) -> EngineIterator {
        EngineIterator::new(self.memtable.iter())
    }

    fn get_c0(&self, k: &Key) -> Option<Value> {
        self.memtable.get(k).cloned()
    }

    fn get_c1(&self, k: &Key) -> Result<Option<Value>> {
        let idx = self.slabs.binary_search_by(|f| f.cmp(&k));

        match idx {
            Ok(idx) => Ok(self.slabs[idx].sstable()?.get(&k)?),
            Err(_) => Ok(None),
        }
    }
}
