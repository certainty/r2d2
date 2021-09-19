use r2d2::engine;
use r2d2::engine::{Engine, Key, Value};
use std::path::PathBuf;

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
