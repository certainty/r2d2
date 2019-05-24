//! The LSM implements a log structured merge tree
//!
//! Docs: https://en.wikipedia.org/wiki/Log-structured_merge-tree
//!
//! The module exposes a struct that can be used to construct
//! an LSM by providing appropriate implementations for the underlying
//! components. The architecture will take care of all aspects
//! related to the management of the local LSM. It might spawn additional
//! threads.

extern crate skiplist;
pub mod commit_log;

use log::{info, trace};
use skiplist::SkipMap;
use std::path::Path;
use std::result;

type Result<T> = result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    CommitLogError(commit_log::Error),
}

impl From<commit_log::Error> for Error {
    fn from(e: commit_log::Error) -> Self {
        Error::CommitLogError(e)
    }
}

type Memtable = SkipMap<Vec<u8>, Vec<u8>>;

pub struct LSM {
    commit_log: commit_log::CommitLogWriter,
    memtable: Memtable,
}

const COMMIT_LOG_NAME: &str = "commit.log";

pub fn init(storage_directory: &Path) -> Result<LSM> {
    let commit_log_path = storage_directory.join(COMMIT_LOG_NAME);

    let lsm = if commit_log_path.exists() {
        init_with_recovery(commit_log_path.as_path())
    } else {
        init_clean(commit_log_path.as_path())
    };

    info!("lsm subsystem initialized and ready");
    lsm
}

fn init_clean(commit_log_path: &Path) -> Result<LSM> {
    let mut memtable = SkipMap::new();

    info!(
        "starting lsm with fresh commit log at {:?}",
        commit_log_path
    );

    Ok(LSM {
        commit_log: commit_log::create(commit_log_path)?,
        memtable,
    })
}

fn init_with_recovery(commit_log_path: &Path) -> Result<LSM> {
    let mut memtable = SkipMap::new();

    info!("commit log exists, will try to repair");
    let mut lsm_for_repair = LSM {
        commit_log: commit_log::null()?,
        memtable,
    };

    recover(&mut lsm_for_repair, commit_log_path)?;
    info!("state is restored successfully");

    Ok(LSM {
        commit_log: commit_log::resume(commit_log_path)?,
        ..lsm_for_repair
    })
}

fn recover(lsm: &mut LSM, commit_log_path: &Path) -> Result<()> {
    info!(
        "repairing local state from commit log at: {:?}",
        commit_log_path
    );

    let mut reader = commit_log::open(commit_log_path)?;

    for result_of_op in reader {
        match result_of_op? {
            commit_log::Operation::Set(key, value) => {
                lsm.set(key, value)?;
                ()
            }
            commit_log::Operation::Delete(key) => {
                lsm.del(key)?;
                ()
            }
        }
    }

    Ok(())
}

impl LSM {
    pub fn set(&mut self, k: Vec<u8>, v: Vec<u8>) -> Result<Option<Vec<u8>>> {
        self.commit_log.write(commit_log::Operation::Set(&k, &v))?;
        Ok(self.memtable.insert(k, v))
    }

    pub fn del(&mut self, k: Vec<u8>) -> Result<Option<Vec<u8>>> {
        self.commit_log.write(commit_log::Operation::Delete(&k))?;
        Ok(self.memtable.remove(&k))
    }

    pub fn get(&self, k: Vec<u8>) -> Result<Option<&Vec<u8>>> {
        Ok(self.memtable.get(&k))
    }

    pub fn keys(&self) -> Result<Vec<&Vec<u8>>> {
        Ok(self.memtable.keys().collect())
    }
}
