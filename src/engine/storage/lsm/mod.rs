pub mod commit_log;

use std::path::Path;
use std::result::Result;
use commit_log::CommitLog;
use log::error;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Error {
  OperationNotCommitedError
}

pub struct LSM {
  commit_log: CommitLog,
  // TODO: synchronize
  memtable: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl LSM {
  pub fn new(storage_directory: &Path) -> LSM {
    let commit_log = CommitLog::new(storage_directory).unwrap();

    LSM {
      commit_log: commit_log,
      memtable: BTreeMap::new(),
    }
  }

  pub fn insert(&mut self, k: &[u8], v: &[u8]) -> Result<(), Error> {
    self.commit_log.commit_set(k,v).map_err(|_| Error::OperationNotCommitedError)?;
    self.memtable.insert(k.to_vec(), v.to_vec());
    Ok(())
  }
}