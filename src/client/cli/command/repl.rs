use crate::client::repl;
use crate::engine;
use clap::Clap;
use std::path::PathBuf;

#[derive(Clap)]
#[clap(version = "0.1", author = "David K.", about = "Start the REPL")]
pub struct Opts {
    #[clap(short, long, about = "the path to the directory for the storage")]
    storage_directory: Option<String>,
}

pub fn execute(opts: &Opts) -> anyhow::Result<()> {
    let storage_config = if let Some(dir) = &opts.storage_directory {
        engine::storage::Configuration::new(PathBuf::from(dir))
    } else {
        engine::storage::Configuration::new(engine::directories::default_storage_path()?)
    };
    let config = engine::configuration::Configuration::new(storage_config);

    let mut engine = engine::Engine::new(config)?;
    repl::run(&mut engine);
    Ok(())
}
