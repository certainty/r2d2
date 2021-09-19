//! The LSM implements a log structured merge tree
//!
//! Docs: https://en.wikipedia.org/wiki/Log-structured_merge-tree
//!
//! The module exposes a struct that can be used to construct
//! an LSM by providing appropriate implementations for the underlying
//! components. The architecture will take care of all aspects
//! related to the management of the local LSM. It might spawn additional
//! threads.

use std::collections::BTreeMap;
use std::path::Path;
use std::result;

use log::info;

mod binary_io;
pub mod sstable;
pub mod wal;
use crate::engine::{Key, Value};
use thiserror::Error;

type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    WalError(#[from] wal::Error),
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
}

type Memtable = BTreeMap<Key, Value>;

pub struct LSM {
    wal: wal::WalWriter,
    memtable: Memtable,
}

pub fn init(base_directory: &Path) -> Result<LSM> {
    let wal = wal::init(base_directory)?;

    let lsm = if wal.recovery_needed() {
        init_with_recovery(&wal)
    } else {
        init_clean(&wal)
    };

    info!(target: "LSM","lsm subsystem initialized and ready");
    lsm
}

fn init_clean(wal: &wal::Wal) -> Result<LSM> {
    let memtable = Memtable::new();

    info!(target: "LSM", "starting lsm with fresh commit log",);

    Ok(LSM {
        wal: wal.create()?,
        memtable,
    })
}

fn init_with_recovery(wal: &wal::Wal) -> Result<LSM> {
    info!(target: "LSM", "starting recovery from WAL");

    let memtable = Memtable::new();
    let mut lsm_for_repair = LSM {
        wal: wal.null()?,
        memtable,
    };

    recover(&mut lsm_for_repair, &wal)?;
    info!(target: "LSM", "recovery completed successfully");

    Ok(LSM {
        wal: wal.resume()?,
        ..lsm_for_repair
    })
}

fn recover(lsm: &mut LSM, wal: &wal::Wal) -> Result<()> {
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

impl LSM {
    pub fn set(&mut self, k: Key, v: Value) -> Result<Option<Value>> {
        self.wal.write(wal::Operation::Set(&k, &v))?;
        Ok(self.memtable.insert(k, v))
    }

    pub fn del(&mut self, k: &Key) -> Result<Option<Value>> {
        self.wal.write(wal::Operation::Delete(k))?;
        Ok(self.memtable.remove(k))
    }

    pub fn get(&self, k: &Key) -> Result<Option<&Value>> {
        Ok(self.memtable.get(k))
    }

    pub fn keys(&self) -> Result<Vec<&Key>> {
        Ok(self.memtable.keys().collect())
    }
}
