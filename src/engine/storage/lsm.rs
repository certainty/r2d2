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

pub fn new(storage_directory: &Path) -> Result<LSM> {
    let commit_log = commit_log::resume(storage_directory.join("commit.log").as_path())?;
    let memtable = SkipMap::new();

    Ok(LSM {
        commit_log: commit_log,
        memtable: memtable,
    })
}

impl LSM {
    pub fn set(&mut self, k: Vec<u8>, v: Vec<u8>) -> Result<()> {
        self.commit_log.write_set(&k, &v)?;
        self.memtable.insert(k, v);
        Ok(())
    }

    pub fn del(&mut self, k: Vec<u8>) -> Result<Option<Vec<u8>>> {
        self.commit_log.write_delete(&k)?;
        Ok(self.memtable.remove(&k))
    }

    pub fn get(&self, k: Vec<u8>) -> Result<Option<&Vec<u8>>> {
        Ok(self.memtable.get(&k))
    }

    pub fn keys(&self) -> Result<Vec<&Vec<u8>>> {
        Ok(self.memtable.keys().collect())
    }
}
