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
pub mod wal;

use log::{info, trace};
use skiplist::SkipMap;
use std::path::{Path, PathBuf};
use std::result;

type Result<T> = result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    CommitLogError(wal::Error),
    IoError(std::io::ErrorKind),
}

impl From<wal::Error> for Error {
    fn from(e: wal::Error) -> Self {
        Error::CommitLogError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e.kind())
    }
}

type Memtable = SkipMap<Vec<u8>, Vec<u8>>;

pub struct LSM {
    commit_log: wal::WalWriter,
    memtable: Memtable,
}

const COMMIT_LOG_NAME: &str = "write_ahead.log";

pub fn init(base_directory: &Path) -> Result<LSM> {
    let commit_log_directory = init_directory(base_directory)?;

    let lsm = if recovery_required(&commit_log_directory) {
        init_with_recovery(&commit_log_directory)
    } else {
        init_clean(&commit_log_directory)
    };

    info!("lsm subsystem initialized and ready");
    lsm
}

fn recovery_required(commit_log_directory: &Path) -> bool {
    let commit_log_path = commit_log_directory.join(COMMIT_LOG_NAME);
    commit_log_path.exists()
}

fn init_directory(storage_path: &Path) -> Result<PathBuf> {
    let commit_log_path = storage_path.join("commit_log");
    std::fs::create_dir_all(&commit_log_path)?;
    Ok(commit_log_path)
}

fn init_clean(commit_log_directory: &Path) -> Result<LSM> {
    let commit_log_path = commit_log_directory.join(COMMIT_LOG_NAME);
    let mut memtable = SkipMap::new();

    info!(
        "starting lsm with fresh commit log at {:?}",
        commit_log_path
    );

    Ok(LSM {
        commit_log: wal::create(&commit_log_path)?,
        memtable,
    })
}

fn init_with_recovery(commit_log_directory: &Path) -> Result<LSM> {
    let commit_log_path = commit_log_directory.join(COMMIT_LOG_NAME);
    let mut memtable = SkipMap::new();

    info!("commit log exists, will try to repair");
    let mut lsm_for_repair = LSM {
        commit_log: wal::null()?,
        memtable,
    };

    recover(&mut lsm_for_repair, &commit_log_path)?;
    info!("state is restored successfully");

    Ok(LSM {
        commit_log: wal::resume(&commit_log_path)?,
        ..lsm_for_repair
    })
}

fn recover(lsm: &mut LSM, commit_log_path: &Path) -> Result<()> {
    info!(
        "repairing local state from commit log at: {:?}",
        commit_log_path
    );

    let mut reader = wal::open(commit_log_path)?;

    for result_of_op in reader {
        match result_of_op? {
            wal::Operation::Set(key, value) => {
                lsm.set(key, value)?;
                ()
            }
            wal::Operation::Delete(key) => {
                lsm.del(key)?;
                ()
            }
        }
    }

    Ok(())
}

impl LSM {
    pub fn set(&mut self, k: Vec<u8>, v: Vec<u8>) -> Result<Option<Vec<u8>>> {
        self.commit_log.write(wal::Operation::Set(&k, &v))?;
        Ok(self.memtable.insert(k, v))
    }

    pub fn del(&mut self, k: Vec<u8>) -> Result<Option<Vec<u8>>> {
        self.commit_log.write(wal::Operation::Delete(&k))?;
        Ok(self.memtable.remove(&k))
    }

    pub fn get(&self, k: Vec<u8>) -> Result<Option<&Vec<u8>>> {
        Ok(self.memtable.get(&k))
    }

    pub fn keys(&self) -> Result<Vec<&Vec<u8>>> {
        Ok(self.memtable.keys().collect())
    }
}
