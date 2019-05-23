use std::fs;

pub static TEST_DIRECTORY: &str = "tmp/r2d2";
pub static TEST_STORAGE_DIRECTORY: &str = "/tmp/r2d2/storage";
pub static TEST_ENGINE_DIRECTORY: &str = "/tmp/r2d2/engine";

pub fn setup() {
    env_logger::try_init();
    fs::remove_dir_all(&TEST_DIRECTORY).unwrap();
    fs::remove_dir_all(&TEST_STORAGE_DIRECTORY).unwrap();
    fs::remove_dir_all(&TEST_ENGINE_DIRECTORY).unwrap();

    fs::create_dir_all(&TEST_DIRECTORY).unwrap();
    fs::create_dir_all(&TEST_STORAGE_DIRECTORY).unwrap();
    fs::create_dir_all(&TEST_ENGINE_DIRECTORY).unwrap();
}

pub fn str_vec(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}
