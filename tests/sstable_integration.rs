extern crate r2d2_lib;

mod utils;

use r2d2_lib::engine::storage::lsm::sstable;
use std::path::{Path, PathBuf};
use utils::*;

#[test]
fn check_write_sstable() {
    setup();

    let mut writer =
        sstable::Writer::create(&Path::new(TEST_STORAGE_DIRECTORY).join("sstable")).unwrap();

    assert!(writer.append(str_vec("foo"), str_vec("bar")).is_ok());
    assert!(writer.append(str_vec("bar"), str_vec("baz")).is_ok());
    assert!(writer.append(str_vec("baz"), str_vec("frooble")).is_ok());

    let slab = writer.seal().unwrap();
    let mut sstable = sstable::SSTable::open(&slab.path).unwrap();

    assert_eq!(Some(str_vec("bar")), sstable.get(&str_vec("foo")).unwrap());
    assert_eq!(Some(str_vec("baz")), sstable.get(&str_vec("bar")).unwrap());
    assert_eq!(None, sstable.get(&str_vec("foobar")).unwrap());
}
