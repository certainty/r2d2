mod utils;
use r2d2::engine;
use r2d2::engine::{Engine, Key, Value, Result};
use std::path::PathBuf;
use utils::*;

#[test]
fn basic_operation_works() -> Result<()> {
    setup();
    let mut ngin = engine::default::new(PathBuf::from(TEST_ENGINE_DIRECTORY));

    assert_eq!(ngin.get(&Key::from_string("foo"))?, None);

    assert_eq!(
        ngin.set(Key::from_string("foo"), Value::from_string("bar"))?,
        None
    );

    assert_eq!(
        ngin.get(&Key::from_string("foo"))?,
        Some(Value::from_string("bar"))
    );

    ngin.set(Key::from_string("foo"), Value::from_string("updated"))?;

    assert_eq!(
        ngin.get(&Key::from_string("foo"))?,
        Some(Value::from_string("updated"))
    );

    assert_eq!(
        ngin.del(&Key::from_string("foo"))?,
        Some(Value::from_string("updated"))
    );

    assert_eq!(ngin.get(&Key::from_string("foo"))?, None);

    Ok(())
}
