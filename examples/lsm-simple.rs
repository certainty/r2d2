use r2d2::engine;
use r2d2::engine::{Engine, Key, Result, Value};
use std::path::PathBuf;

fn main() -> Result<()> {
    env_logger::init();
    let mut engine = engine::lsm_engine::new(PathBuf::from("/tmp"));

    engine.set("Foo", "this is the value I want to store")?;
    engine.set("Bar", "some other value")?;

    println!("Value: {:?}", engine.get(&Key::from("foo"))?);
    Ok(())
}
