extern crate r2d2_lib;
use r2d2_lib::engine;
use r2d2_lib::engine::{Engine, Key, Value};
use std::path::PathBuf;

#[test]
fn basic_operation_works() {
  env_logger::init();
  let mut ngin = engine::default::new(PathBuf::from("/tmp"));

  assert_eq!(ngin.lookup(Key::from_string("foo")), Ok(None));

  assert_eq!(
    ngin.insert(Key::from_string("foo"), Value::from_string("bar")),
    Ok(None)
  );

  assert_eq!(
    ngin.lookup(Key::from_string("foo")),
    Ok(Some(Value::from_string("bar")))
  );

  ngin
    .insert(Key::from_string("foo"), Value::from_string("updated"))
    .unwrap();

  assert_eq!(
    ngin.lookup(Key::from_string("foo")),
    Ok(Some(Value::from_string("updated")))
  );

  assert_eq!(
    ngin.delete(Key::from_string("foo")),
    Ok(Some(Value::from_string("updated")))
  );

  assert_eq!(ngin.lookup(Key::from_string("foo")), Ok(None));
}
