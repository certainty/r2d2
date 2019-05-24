extern crate r2d2_lib;

mod utils;

use r2d2_lib::engine::storage::lsm::wal;
use tempfile;
use utils::*;
use wal::Operation;

#[test]
fn check_commit_log_works() {
    setup();
    let file = tempfile::NamedTempFile::new_in(TEST_STORAGE_DIRECTORY).unwrap();
    let mut log_writer = wal::create(file.path()).unwrap();
    let mut log_reader = wal::open(file.path()).unwrap();
    let foo = str_vec("foo");
    let bar = str_vec("bar");
    let baz = str_vec("baz");

    assert!(log_writer.write(Operation::Set(&foo, &bar)).is_ok());
    assert!(log_writer.write(Operation::Set(&bar, &baz)).is_ok());
    assert!(log_writer.write(Operation::Delete(&bar)).is_ok());

    let op1 = log_reader.read().unwrap();
    assert_eq!(Operation::Set(foo.clone(), bar.clone()), op1);

    let op2 = log_reader.read().unwrap();
    assert_eq!(Operation::Set(bar.clone(), baz.clone()), op2);

    let op3 = log_reader.read().unwrap();
    assert_eq!(Operation::Delete(bar.clone()), op3);
}

#[test]
fn check_commit_log_iterator() {
    setup();
    let file = tempfile::NamedTempFile::new_in(TEST_STORAGE_DIRECTORY).unwrap();
    let mut log_writer = wal::create(file.path()).unwrap();
    let mut log_reader = wal::open(file.path()).unwrap();
    let foo = str_vec("foo");
    let bar = str_vec("bar");
    let baz = str_vec("baz");

    assert!(log_writer.write(Operation::Set(&foo, &bar)).is_ok());
    assert!(log_writer.write(Operation::Set(&bar, &baz)).is_ok());
    assert!(log_writer.write(Operation::Delete(&bar)).is_ok());

    let op1 = log_reader.next().unwrap().unwrap();
    assert_eq!(wal::Operation::Set(foo.clone(), bar.clone()), op1);

    let op2 = log_reader.next().unwrap().unwrap();
    assert_eq!(wal::Operation::Set(bar.clone(), baz.clone()), op2);

    let op3 = log_reader.next().unwrap().unwrap();
    assert_eq!(wal::Operation::Delete(bar.clone()), op3);

    assert!(log_reader.next().is_none());
}

#[test]
fn check_iterator_empty_file() {
    setup();

    let file = tempfile::NamedTempFile::new_in(TEST_STORAGE_DIRECTORY).unwrap();
    let _log_writer = wal::create(file.path()).unwrap();
    let mut log_reader = wal::open(file.path()).unwrap();

    assert!(log_reader.next().is_none());
}

#[test]
fn check_log_resume() {
    setup();
    let file = tempfile::NamedTempFile::new_in(TEST_STORAGE_DIRECTORY).unwrap();
    let foo = str_vec("foo");
    let bar = str_vec("bar");
    let foobar = str_vec("foobar");

    {
        let mut log_writer = wal::create(file.path()).unwrap();
        assert!(log_writer.write(Operation::Set(&foo, &bar)).is_ok());
    }

    {
        let mut log_writer = wal::resume(file.path()).unwrap();
        assert!(log_writer.write(Operation::Set(&foobar, &bar)).is_ok());
    }

    let mut log_reader = wal::open(file.path()).unwrap();
    let op1 = log_reader.next().unwrap().unwrap();
    assert_eq!(wal::Operation::Set(foo.clone(), bar.clone()), op1);

    let op2 = log_reader.next().unwrap().unwrap();
    assert_eq!(wal::Operation::Set(foobar.clone(), bar.clone()), op2);
}
