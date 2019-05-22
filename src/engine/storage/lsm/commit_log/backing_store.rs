//! A backing store is a small abstraction that hands
//! out a reader and a writer the same underlying storage

extern crate byteorder;
use super::Error;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub trait BackingStore {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error>;
    fn write(&mut self, data: &[u8]) -> Result<usize, std::io::Error>;
}

// Most common backing store is a file based backing store
pub struct FileBackingStore(File, File);

impl FileBackingStore {
    pub fn new(path: &Path) -> Result<FileBackingStore, Error> {
        let writer = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path)?;

        let reader = OpenOptions::new().read(true).open(path)?;

        Ok(FileBackingStore(reader, writer))
    }
}

impl BackingStore for FileBackingStore {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        read_frame(&mut self.0, buf)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, std::io::Error> {
        write_frame(&mut self.1, data)
    }
}

// utility functions that can be used to read and write frames
fn read_frame(reader: &mut Read, buf: &mut [u8]) -> Result<usize, std::io::Error> {
    let size = reader.read_u64::<LittleEndian>()?;

    reader.take(size).read(buf)
}

fn write_frame(writer: &mut Write, data: &[u8]) -> Result<usize, std::io::Error> {
    writer.write_u64::<LittleEndian>(data.len() as u64)?;
    writer.write(data)
}
