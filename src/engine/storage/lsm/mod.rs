pub mod commit_log;

use std::path::{PathBuf};
use std::result::Result;
use commit_log::CommitLog;

pub struct LSM {
  commit_log: CommitLog,
}

impl LSM {
  pub fn new(storage_directory: PathBuf) -> LSM {
    let commit_log = CommitLog::new(storage_directory).unwrap();

    LSM {
      commit_log: commit_log
    }
  }

  pub fn insert(&mut self, k: &Vec<u8>, v: &Vec<u8>) -> Result<(), String> {
    Ok(())
  }
}