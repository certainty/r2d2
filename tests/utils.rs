#[allow(unused_must_use)]
use std::fs;

pub static TEST_DIRECTORY: &str = "tmp/r2d2/tests";
pub static TEST_STORAGE_DIRECTORY: &str = "/tmp/r2d2/tests/storage";
pub static TEST_ENGINE_DIRECTORY: &str = "/tmp/r2d2/tests/engine";

pub fn setup() {
    let directories = [
        TEST_DIRECTORY,
        TEST_ENGINE_DIRECTORY,
        TEST_STORAGE_DIRECTORY,
    ];

    for dir in directories {
        let path = std::path::PathBuf::from(dir);
        if path.is_dir() {
            fs::remove_dir_all(&path).unwrap();
        }

        fs::create_dir_all(&path).unwrap();
    }
}

#[cfg(test)]
pub fn str_vec(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}
