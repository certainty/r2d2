use r2d2::engine;
use r2d2::engine::{Engine, Key, Result, Value};
use std::path::PathBuf;

fn main() -> Result<()> {
    env_logger::init();
    let config = engine::configuration::Configuration::new(engine::storage::Configuration::new(
        PathBuf::from("/tmp"),
    ));
    let mut engine = engine::Engine::new(config)?;
    engine.set("Foo", "this is the value I want to store")?;
    engine.set("Bar", "some other value")?;

    println!("Value: {:?}", engine.get(&Key::from("foo"))?);
    Ok(())
}
