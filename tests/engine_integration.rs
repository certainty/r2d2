extern crate r2d2_lib;
use r2d2_lib::engine::Engine;
use r2d2_lib::engine;

#[test]
fn basic_operation_works() {
    env_logger::init();
    let mut ngin = engine::default::new();

    assert_eq!(
      ngin.lookup(String::from("foo")),
      Ok(None));

    assert_eq!(
      ngin.insert(String::from("foo"), String::from("bar")),
      Ok(None));

    assert_eq!(
      ngin.lookup(String::from("foo")),
      Ok(Some(&String::from("bar"))));

    assert_eq!(
      ngin.delete(String::from("foo")),
      Ok(Some(String::from("bar"))));

    assert_eq!(
      ngin.lookup(String::from("foo")),
      Ok(None));
    
}
