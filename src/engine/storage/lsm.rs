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

use commit_log::backing_store::FileBackingStore;
use commit_log::CommitLog;
use skiplist::SkipMap;
use std::path::Path;
use std::result::Result;

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
    commit_log: CommitLog,
    memtable: Memtable,
}

impl LSM {
    pub fn new(storage_directory: &Path) -> LSM {
        let backing_store = FileBackingStore::new(storage_directory).unwrap();
        let commit_log = CommitLog::new(backing_store).unwrap();
        let memtable = SkipMap::new();

        LSM {
            commit_log: commit_log,
            memtable: memtable,
        }
    }

    pub fn insert(&mut self, k: Vec<u8>, v: Vec<u8>) -> Result<(), Error> {
        self.commit_log.commit_set(k.as_slice(), v.as_slice())?;
        self.memtable.insert(k, v);
        Ok(())
    }

    pub fn remove(&mut self, k: Vec<u8>) -> Result<Option<Vec<u8>>, Error> {
        self.commit_log.commit_delete(k.as_slice())?;
        Ok(self.memtable.remove(&k))
    }

    pub fn lookup(&self, k: Vec<u8>) -> Result<Option<&Vec<u8>>, Error> {
        Ok(self.memtable.get(&k))
    }

    pub fn keys(&self) -> Result<Vec<&Vec<u8>>, Error> {
        Ok(self.memtable.keys().collect())
    }
}
