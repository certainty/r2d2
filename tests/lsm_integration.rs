extern crate r2d2_lib;

mod utils;

use r2d2_lib::engine::storage::lsm;
use std::path::Path;
use tempfile;
use utils::*;

#[test]
fn check_lsm_works() {
    setup();
    let mut lsm = lsm::new(Path::new(TEST_STORAGE_DIRECTORY)).unwrap();
    let foo = str_vec("foo");
    let bar = str_vec("bar");
    let baz = str_vec("baz");

    assert!(lsm.get(foo.clone()).unwrap().is_none());
    assert!(lsm.set(foo.clone(), bar.clone()).is_ok());
    assert_eq!(Some(&bar.clone()), lsm.get(foo.clone()).unwrap());
}

#[test]
fn check_recovery_from_commit_log() {
    setup();
    let foo = str_vec("foo");
    let bar = str_vec("bar");
    let baz = str_vec("baz");

    {
        let mut lsm = lsm::new(Path::new(TEST_STORAGE_DIRECTORY)).unwrap();

        assert!(lsm.set(foo.clone(), bar.clone()).is_ok());
        assert!(lsm.set(bar.clone(), baz.clone()).is_ok());
    }

    // now open a new LSM that recreates the state from the commit log
    let lsm = lsm::new(Path::new(TEST_STORAGE_DIRECTORY)).unwrap();

    // keys should be there now
    assert_eq!(Some(&bar.clone()), lsm.get(foo.clone()).unwrap());
    assert_eq!(Some(&baz.clone()), lsm.get(bar.clone()).unwrap());
}
