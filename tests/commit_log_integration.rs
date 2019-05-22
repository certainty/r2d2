extern crate r2d2_lib;

mod utils;

use r2d2_lib::engine::storage::lsm::commit_log;
use r2d2_lib::engine::storage::lsm::commit_log::backing_store;
use tempfile;
use tempfile::NamedTempFile;
use utils::*;

#[test]
fn commiting_operations() {
    setup();

    let f = NamedTempFile::new_in(TEST_STORAGE_DIRECTORY).unwrap();
    let store = backing_store::FileBackingStore::new(f.path()).unwrap();
    let mut log = commit_log::CommitLog::new(store).unwrap();

    assert!(log.commit_set("foo".as_bytes(), "bar".as_bytes()).is_ok());
    assert!(log
        .commit_set("baz".as_bytes(), "foobar".as_bytes())
        .is_ok());

    assert!(log.commit_delete("baz".as_bytes()).is_ok());

    log.each_operation(|op| println!("Op: {:?}", op))
}
