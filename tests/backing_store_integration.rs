extern crate r2d2_lib;

mod utils;
use r2d2_lib::engine::storage::lsm::commit_log::backing_store;
use std::fs::OpenOptions;
use std::path::Path;
use tempfile;
use tempfile::NamedTempFile;
use utils::*;

#[test]
fn write_data_works() {
    setup();

    let f = NamedTempFile::new_in(TEST_STORAGE_DIRECTORY).unwrap();
    let store = backing_store::FileBackingStore::new(f.path()).unwrap();
    assert_eq!(0, 0);
}

#[test]
fn read_data_works() {
    setup();
}
