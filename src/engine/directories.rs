use crate::engine::configuration::Configuration;
use directories;
use directories::ProjectDirs;
use std::path::PathBuf;
use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("System doesn't have project directories")]
    MissingProjectDirectory,
    #[error("Could not initialise paths")]
    PathSetupError,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub fn setup_directories(config: &Configuration) -> Result<()> {
    if !config.storage.storage_path.is_dir() {
        std::fs::create_dir_all(&config.storage.storage_path)?;
    }
    Ok(())
}

pub fn project_directories() -> Result<ProjectDirs> {
    match directories::ProjectDirs::from("de", "lisp-unleashed", "r2d2") {
        Some(d) => Ok(d),
        None => Err(Error::MissingProjectDirectory),
    }
}

pub fn default_storage_path() -> Result<PathBuf> {
    project_directories().map(|d| d.data_dir().to_path_buf())
}

pub fn default_config_path() -> Result<PathBuf> {
    project_directories().map(|d| d.config_dir().join("r2d2.toml"))
}
