extern crate r2d2_lib;
use r2d2_lib::engine;
use r2d2_lib::engine::{Engine, Key, Value};

#[test]
fn basic_operation_works() {
  env_logger::init();
  let mut ngin = engine::default::new();

  assert_eq!(ngin.lookup(Key::from_str("foo")), Ok(None));

  assert_eq!(
    ngin.insert(Key::from_str("foo"), Value::from_str("bar")),
    Ok(None)
  );

  assert_eq!(
    ngin.lookup(Key::from_str("foo")),
    Ok(Some(&Value::from_str("bar")))
  );

  ngin
    .insert(Key::from_str("foo"), Value::from_str("updated"))
    .unwrap();

  assert_eq!(
    ngin.lookup(Key::from_str("foo")),
    Ok(Some(&Value::from_str("updated")))
  );

  assert_eq!(
    ngin.delete(Key::from_str("foo")),
    Ok(Some(Value::from_str("updated")))
  );

  assert_eq!(ngin.lookup(Key::from_str("foo")), Ok(None));
}
