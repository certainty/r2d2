mod utils;

use r2d2::engine::storage::lsm;
use r2d2::engine::{Key, Value};
use std::path::Path;
use utils::*;

#[test]
fn check_lsm_works() {
    setup();
    let mut lsm = lsm::init(Path::new(TEST_STORAGE_DIRECTORY)).unwrap();
    let foo = Key::from("foo");
    let bar = Value::from("bar");

    assert!(lsm.get(&foo).unwrap().is_none());
    assert!(lsm.set(foo.clone(), bar.clone()).is_ok());
    assert_eq!(Some(&bar.clone()), lsm.get(&foo).unwrap());
}

#[test]
fn check_recovery_from_commit_log() {
    setup();
    let foo = Key::from("foo");
    let bar = Key::from("bar");
    let baz = Value::from("baz");

    {
        let mut lsm = lsm::init(Path::new(TEST_STORAGE_DIRECTORY)).unwrap();

        assert!(lsm.set(foo.clone(), baz.clone()).is_ok());
        assert!(lsm.set(bar.clone(), baz.clone()).is_ok());
    }

    // now open a new LSM that recreates the state from the commit log
    let lsm = lsm::init(Path::new(TEST_STORAGE_DIRECTORY)).unwrap();

    // keys should be there now
    assert_eq!(Some(&baz.clone()), lsm.get(&foo).unwrap());
    assert_eq!(Some(&baz.clone()), lsm.get(&bar).unwrap());
}
