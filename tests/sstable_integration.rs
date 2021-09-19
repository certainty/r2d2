mod utils;

use r2d2::engine::storage::lsm::sstable;
use r2d2::engine::{Key, Value};
use std::path::Path;
use tempfile::tempdir;
use utils::*;

#[test]
fn check_write_sstable() {
    let test_storage_dir = tempdir().unwrap();
    let mut writer =
        sstable::Writer::create(&test_storage_dir.path().to_path_buf().join("sstable")).unwrap();

    assert!(writer
        .append(&Key::from("foo"), &Value::from("bar"))
        .is_ok());
    assert!(writer
        .append(&Key::from("bar"), &Value::from("baz"))
        .is_ok());
    assert!(writer
        .append(&Key::from("baz"), &Value::from("frooble"))
        .is_ok());

    let slab = writer.seal().unwrap();
    let mut sstable = slab.sstable().unwrap();

    assert_eq!(
        sstable.get(&Key::from("foo")).unwrap(),
        Some(Value::from("bar")),
    );
    assert_eq!(
        sstable.get(&Key::from("bar")).unwrap(),
        Some(Value::from("baz"))
    );
    assert_eq!(sstable.get(&Key::from("foobar")).unwrap(), None);
}
