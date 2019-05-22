extern crate r2d2_lib;

mod utils;
use r2d2_lib::engine::storage::lsm::commit_log::backing_store;
use r2d2_lib::engine::storage::lsm::commit_log::backing_store::BackingStore;
use tempfile;
use tempfile::NamedTempFile;
use utils::*;

#[test]
fn read_your_own_writes() {
    setup();

    let f = NamedTempFile::new_in(TEST_STORAGE_DIRECTORY).unwrap();
    let mut store = backing_store::FileBackingStore::new(f.path()).unwrap();
    let frame1 = String::from("this is my first frame of data");
    let frame2 = String::from("this is the second frame");
    let mut read_frame1 = Vec::with_capacity(frame1.len());
    let mut read_frame2 = Vec::with_capacity(frame2.len());

    assert!(store.write(frame1.as_bytes()).is_ok(), "can write data");
    assert!(store.write(frame2.as_bytes()).is_ok(), "can write data");

    store.rewind_for_read();
    assert!(store.read(&mut read_frame1).is_ok(), "can read data back");
    assert!(store.read(&mut read_frame2).is_ok(), "can read data back");

    assert_eq!(frame1.as_bytes(), read_frame1.as_slice());
    assert_eq!(frame2.as_bytes(), read_frame2.as_slice());
}

#[test]
fn read_is_independent_of_write() {
    let f = NamedTempFile::new_in(TEST_STORAGE_DIRECTORY).unwrap();
    let mut store = backing_store::FileBackingStore::new(f.path()).unwrap();
    let frame1 = String::from("this is my first frame of data");
    let frame2 = String::from("this is the second frame");
    let mut buf = Vec::with_capacity(100);
    let mut read_frame1 = Vec::with_capacity(frame1.len());
    let mut read_frame2 = Vec::with_capacity(frame2.len());

    // first write
    assert!(store.write(frame1.as_bytes()).is_ok());

    // this should not interfere with the write
    store.rewind_for_read();
    assert!(store.read(&mut buf).is_ok());

    // second write
    assert!(store.write(frame2.as_bytes()).is_ok());

    store.rewind_for_read();
    assert!(store.read(&mut read_frame1).is_ok(), "can read data back");
    assert!(store.read(&mut read_frame2).is_ok(), "can read data back");

    assert_eq!(frame1.as_bytes(), read_frame1.as_slice());
    assert_eq!(frame2.as_bytes(), read_frame2.as_slice());
}
