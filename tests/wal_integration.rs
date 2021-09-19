mod utils;

use r2d2::engine::storage::lsm::wal;
use r2d2::engine::{Key, Value};
use std::path::Path;
use tempfile::tempdir;
use utils::*;
use wal::Operation;

#[test]
fn check_wal_works() {
    let test_storage_dir = tempdir().unwrap();
    let wal = wal::init(&test_storage_dir.path().to_path_buf()).unwrap();
    let mut log_writer = wal.create().unwrap();
    let mut log_reader = wal.open().unwrap();
    let foo = Key::from("foo");
    let bar = Value::from("bar");
    let baz = Value::from("baz");

    assert!(log_writer.write(Operation::Set(&foo, &bar)).is_ok());
    assert!(log_writer.write(Operation::Set(&foo, &baz)).is_ok());
    assert!(log_writer.write(Operation::Delete(&foo)).is_ok());

    let op1 = log_reader.read().unwrap();
    assert_eq!(Operation::Set(foo.clone(), bar.clone()), op1);

    let op2 = log_reader.read().unwrap();
    assert_eq!(Operation::Set(foo.clone(), baz.clone()), op2);

    let op3 = log_reader.read().unwrap();
    assert_eq!(Operation::Delete(foo.clone()), op3);
}

#[test]
fn check_wal_iterator() {
    let test_storage_dir = tempdir().unwrap();
    let wal = wal::init(&test_storage_dir.path().to_path_buf()).unwrap();
    let mut log_writer = wal.create().unwrap();
    let foo = Key::from("foo");
    let baz = Value::from("baz");
    let bar = Value::from("bar");

    assert!(log_writer.write(Operation::Set(&foo, &bar)).is_ok());
    assert!(log_writer.write(Operation::Set(&foo, &baz)).is_ok());
    assert!(log_writer.write(Operation::Delete(&foo)).is_ok());

    let mut log_reader = wal.open().unwrap();
    let op1 = log_reader.next().unwrap().unwrap();
    assert_eq!(wal::Operation::Set(foo.clone(), bar.clone()), op1);

    let op2 = log_reader.next().unwrap().unwrap();
    assert_eq!(wal::Operation::Set(foo.clone(), baz.clone()), op2);

    let op3 = log_reader.next().unwrap().unwrap();
    assert_eq!(wal::Operation::Delete(foo.clone()), op3);

    assert!(log_reader.next().is_none());
}

#[test]
fn check_iterator_empty_file() {
    let test_storage_dir = tempdir().unwrap();
    let wal = wal::init(&test_storage_dir.path().to_path_buf()).unwrap();
    let _log_writer = wal.create().unwrap();
    let mut log_reader = wal.open().unwrap();

    assert!(log_reader.next().is_none());
}

#[test]
fn check_log_resume() {
    let test_storage_dir = tempdir().unwrap();
    let wal = wal::init(&test_storage_dir.path().to_path_buf()).unwrap();
    let foo = Key::from("foo");
    let foobar = Key::from("foobar");
    let bar = Value::from("bar");

    {
        let mut log_writer = wal.create().unwrap();
        assert!(log_writer.write(Operation::Set(&foo, &bar)).is_ok());
    }

    {
        let mut log_writer = wal.resume().unwrap();
        assert!(log_writer.write(Operation::Set(&foobar, &bar)).is_ok());
    }

    let mut log_reader = wal.open().unwrap();
    let op1 = log_reader.next().unwrap().unwrap();
    assert_eq!(wal::Operation::Set(foo.clone(), bar.clone()), op1);

    let op2 = log_reader.next().unwrap().unwrap();
    assert_eq!(wal::Operation::Set(foobar.clone(), bar.clone()), op2);
}
