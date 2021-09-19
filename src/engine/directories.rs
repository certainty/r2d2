use directories;
use directories::ProjectDirs;
use std::path::PathBuf;
use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Error)]
pub enum Error {
    #[error("System doesn't have project directories")]
    MissingProjectDirectory,
    #[error("Could not initialise paths")]
    PathSetupError,
}

pub fn setup_directories() -> Result<()> {
    let project_dirs = project_directories()?;

    if !project_dirs.data_dir().is_dir() {
        std::fs::create_dir_all(project_dirs.data_dir()).unwrap();
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
