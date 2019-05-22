use std::fs;

pub static TEST_DIRECTORY: &str = "tmp/r2d2";
pub static TEST_STORAGE_DIRECTORY: &str = "/tmp/r2d2/storage";
pub static TEST_ENGINE_DIRECTORY: &str = "/tmp/r2d2/engine";

pub fn setup() {
    fs::create_dir_all(&TEST_DIRECTORY).unwrap();
    fs::create_dir_all(&TEST_STORAGE_DIRECTORY).unwrap();
    fs::create_dir_all(&TEST_ENGINE_DIRECTORY).unwrap();
}
