mod utils;
use r2d2::engine;
use r2d2::engine::{Engine, Key, Result, Value};
use std::path::PathBuf;
use utils::*;

#[test]
fn basic_operation_works() -> Result<()> {
    setup();
    let mut ngin = engine::default::new(PathBuf::from(TEST_ENGINE_DIRECTORY));

    assert_eq!(ngin.get(&Key::from("foo"))?, None);

    assert_eq!(ngin.set("foo", "bar")?, None);

    assert_eq!(ngin.get(&Key::from("foo"))?, Some(Value::from("bar")));

    ngin.set(Key::from("foo"), Value::from("updated"))?;

    assert_eq!(ngin.get(&Key::from("foo"))?, Some(Value::from("updated")));

    assert_eq!(ngin.del(&Key::from("foo"))?, Some(Value::from("updated")));

    assert_eq!(ngin.get(&Key::from("foo"))?, None);

    Ok(())
}
