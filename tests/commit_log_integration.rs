extern crate r2d2_lib;

mod utils;

use r2d2_lib::engine::storage::lsm::commit_log;
use tempfile;
use utils::*;

fn str_vec(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

#[test]
fn check_commit_log_works() {
    setup();
    let file = tempfile::NamedTempFile::new_in(TEST_STORAGE_DIRECTORY).unwrap();
    let mut log_writer = commit_log::create(file.path()).unwrap();
    let mut log_reader = commit_log::open(file.path()).unwrap();
    let foo = str_vec("foo");
    let bar = str_vec("bar");
    let baz = str_vec("baz");

    assert!(log_writer.write_set(&foo, &bar).is_ok());
    assert!(log_writer.write_set(&bar, &baz).is_ok());
    assert!(log_writer.write_delete(&bar).is_ok());

    let op1 = log_reader.read().unwrap();
    assert_eq!(commit_log::Operation::Set(foo.clone(), bar.clone()), op1);

    let op2 = log_reader.read().unwrap();
    assert_eq!(commit_log::Operation::Set(bar.clone(), baz.clone()), op2);

    let op3 = log_reader.read().unwrap();
    assert_eq!(commit_log::Operation::Delete(bar.clone()), op3);
}
