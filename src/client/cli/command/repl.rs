use crate::client::repl;
use crate::engine::{configuration, directories, Engine};
use clap::Clap;
use std::path::PathBuf;

#[derive(Clap)]
#[clap(version = "0.1", author = "David K.", about = "Start the REPL")]
pub struct Opts {
    #[clap(short, long, about = "the path to the directory for the storage")]
    storage_directory: Option<String>,
}

pub fn execute(opts: &Opts) -> anyhow::Result<()> {
    let config = configure(opts)?;
    let mut engine = Engine::start(config)?;
    repl::run(&mut engine);
    Ok(())
}

fn configure(opts: &Opts) -> anyhow::Result<configuration::Configuration> {
    let mut configuration_builder = configuration::Builder::default();

    let storage_base_path = match &opts.storage_directory {
        Some(path) => PathBuf::from(path),
        _ => directories::default_storage_path()?,
    };

    configuration_builder
        .storage
        .with_storage_path(storage_base_path)?;
    let config = configuration_builder.build()?;

    Ok(config)
}
