use r2d2::engine;
use r2d2::engine::{Key, Result};
use std::path::PathBuf;

fn main() -> Result<()> {
    env_logger::init();
    let mut config_builder = engine::configuration::Builder::default();
    config_builder.storage.with_storage_path("/tmp")?;

    let mut engine = engine::Engine::new(config_builder.build()?)?;
    engine.set("Foo", "this is the value I want to store")?;
    engine.set("Bar", "some other value")?;

    println!("Value: {:?}", engine.get(&Key::from("foo"))?);
    Ok(())
}
