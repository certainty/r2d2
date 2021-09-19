mod utils;
use r2d2::engine;
use r2d2::engine::storage;
use r2d2::engine::{Engine, Key, Result, Value};
use std::path::PathBuf;
use tempfile::tempdir;
use utils::*;

#[test]
fn basic_operation_works() -> Result<()> {
    let test_storage_dir = tempdir().unwrap();
    let config = engine::configuration::Configuration::new(storage::Configuration::new(
        test_storage_dir.path().to_path_buf(),
    ));
    let mut ngin = engine::Engine::new(config)?;

    assert_eq!(ngin.get(&Key::from("foo"))?, None);

    assert_eq!(ngin.set("foo", "bar")?, None);

    assert_eq!(ngin.get(&Key::from("foo"))?, Some(&Value::from("bar")));

    ngin.set(Key::from("foo"), Value::from("updated"))?;

    assert_eq!(ngin.get(&Key::from("foo"))?, Some(&Value::from("updated")));

    assert_eq!(ngin.del(&Key::from("foo"))?, Some(Value::from("updated")));

    assert_eq!(ngin.get(&Key::from("foo"))?, None);

    Ok(())
}
