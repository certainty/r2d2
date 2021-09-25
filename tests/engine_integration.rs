use r2d2::engine;
use r2d2::engine::{Key, Value};
use tempfile::tempdir;

#[test]
fn basic_operation_works() -> anyhow::Result<()> {
    let mut config_builder = engine::configuration::Builder::default();
    config_builder
        .storage
        .with_storage_path(tempdir()?.path().to_path_buf())?;
    let config = config_builder.build()?;
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
