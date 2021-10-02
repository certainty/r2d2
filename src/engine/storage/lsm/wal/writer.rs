use super::binio;
use super::Result;
use crate::engine::storage::lsm::wal::serialization::FileHeader;
use crate::engine::storage::lsm::wal::Operation;
use crate::engine::{Key, Value};
use std::io::{BufWriter, Write};
use std::{fs, io, path};

const VERSION: u8 = 1;
const STANZA: &str = "r2d2::wal";

/// The WalWriter is the main interface you will interact with.
pub struct WalWriter {
    file: io::BufWriter<fs::File>,
}

impl WalWriter {
    pub fn resume(path: &path::Path) -> Result<Self> {
        let writer = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(path)?;

        Ok(WalWriter {
            file: BufWriter::new(writer),
        })
    }

    pub fn null() -> Result<Self> {
        let writer = fs::OpenOptions::new()
            .append(true)
            .write(true)
            .open(path::Path::new("/dev/null"))?;

        Ok(WalWriter {
            file: BufWriter::new(writer),
        })
    }

    pub fn create(path: &path::Path) -> Result<WalWriter> {
        let mut writer = fs::OpenOptions::new().create(true).write(true).open(path)?;
        let header = FileHeader::new(STANZA, VERSION);

        binio::write_data(&mut writer, header)?;

        Ok(WalWriter {
            file: BufWriter::new(writer),
        })
    }

    pub fn write(&mut self, op: Operation<&Key, &Value>) -> Result<usize> {
        let size = binio::write_data(&mut self.file, op)?;
        self.file.flush()?;

        Ok(size)
    }
}
