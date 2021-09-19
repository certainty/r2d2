use crate::client::repl;
use crate::engine;
use crate::engine::configuration::Configuration;
use clap::Clap;
use directories;
use std::path::PathBuf;

#[derive(Clap)]
#[clap(version = "0.1", author = "David K.", about = "Start the REPL")]
pub struct Opts {
    #[clap(short, long, about = "the path to the directory for the storage")]
    storage_directory: Option<String>,
}

pub fn execute(opts: &Opts) -> anyhow::Result<()> {
    let mut config = Configuration::default()?;

    if let Some(dir) = &opts.storage_directory {
        config.storage.storage_path = PathBuf::from(dir.clone());
    }

    let mut engine = engine::Engine::new(default_config)?;
    repl::run(&mut engine);
    Ok(())
}

// TODO: move the creation of the directory out of here
fn storage_directory(opts: &Opts) -> PathBuf {
    let project_dirs = directories::ProjectDirs::from("de", "lisp-unleashed", "rd2d").unwrap();

    if !project_dirs.data_dir().is_dir() {
        std::fs::create_dir_all(project_dirs.data_dir()).unwrap();
    }

    match &opts.storage_directory {
        Some(dir) => PathBuf::from(dir),
        None => project_dirs.data_dir().to_path_buf(),
    }
}
