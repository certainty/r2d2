use std::path::PathBuf;
extern crate r2d2_lib;
use r2d2_lib::engine;
use r2d2_lib::engine::{Engine, Key, Value};

fn main() {
    env_logger::init();
    let mut engine = engine::default::new(PathBuf::from("/tmp"));

    engine
        .set(
            Key::from_string("Foo"),
            Value::from_string("this is the value I want to store"),
        )
        .unwrap();

    engine
        .set(
            Key::from_string("Bar"),
            Value::from_string("some other value"),
        )
        .unwrap();

    println!("Value: {:?}", engine.get(&Key::from_string("foo")).unwrap())
}
