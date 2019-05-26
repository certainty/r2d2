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

    assert!(writer.add_data(str_vec("foo"), str_vec("bar")).is_ok());
    assert!(writer.add_data(str_vec("bar"), str_vec("baz")).is_ok());
    assert!(writer.add_data(str_vec("baz"), str_vec("frooble")).is_ok());

    let slab = writer.finish().unwrap();
    println!("SSTABLE written");
    let sstable = sstable::SSTable::open(&slab.path).unwrap();

    assert_eq!(Some(&str_vec("bar")), sstable.get(&str_vec("foo")).unwrap())
}
