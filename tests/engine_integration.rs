extern crate r2d2_lib;
mod utils;
use r2d2_lib::engine;
use r2d2_lib::engine::{Engine, Key, Value};
use std::path::PathBuf;
use utils::*;

#[test]
fn basic_operation_works() {
    setup();
    let mut ngin = engine::default::new(PathBuf::from(TEST_ENGINE_DIRECTORY));

    assert_eq!(ngin.get(&Key::from_string("foo")), Ok(None));

    assert_eq!(
        ngin.set(Key::from_string("foo"), Value::from_string("bar")),
        Ok(None)
    );

    assert_eq!(
        ngin.get(&Key::from_string("foo")),
        Ok(Some(Value::from_string("bar")))
    );

    ngin.set(Key::from_string("foo"), Value::from_string("updated"))
        .unwrap();

    assert_eq!(
        ngin.get(&Key::from_string("foo")),
        Ok(Some(Value::from_string("updated")))
    );

    assert_eq!(
        ngin.del(&Key::from_string("foo")),
        Ok(Some(Value::from_string("updated")))
    );

    assert_eq!(ngin.get(&Key::from_string("foo")), Ok(None));
}
