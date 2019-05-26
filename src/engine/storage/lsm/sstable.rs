use super::binary_io as binio;
use log::trace;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufReader, Seek, SeekFrom, Write};
use std::net::Shutdown::Read;
use std::path;

type Key = Vec<u8>;
type Value = Vec<u8>;
type Offset = usize;
type Level = u8;
type Result<T> = std::result::Result<T, Error>;

const STANZA: &str = "r2d2::sstable";

#[derive(PartialEq, Debug)]
pub enum Error {
    IoError(io::ErrorKind),
    BinIoError(binio::Error),
    EmptyTable,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e.kind())
    }
}

impl From<binio::Error> for Error {
    fn from(e: binio::Error) -> Self {
        Error::BinIoError(e)
    }
}

#[derive(Debug)]
pub struct Slab {
    level: Level,
    max_key: Key,
    min_key: Key,
    path: path::PathBuf,
}

pub struct SSTable {
    // TODO: think about using a trie instead?
    index: HashMap<Key, Offset>,
    path: path::PathBuf,
}

impl SSTable {
    pub fn get(&self, k: &Key) -> Result<Option<&Value>> {
        let offset = self.index.get(k);
        Ok(None)
    }

    pub fn open(path: &path::Path) -> Result<SSTable> {
        unimplemented!()
    }

    fn new(path: &path::Path) -> SSTable {
        SSTable {
            index: HashMap::new(),
            path: path.to_owned(),
        }
    }
}

// On disk representation of SSTable as runs of sorted data:
// The following is a depiction of the table format
//
// DATA_BLOCK
//   key_size key value_length value
//   ...
// META_BLOCK
//   meta_size data
// INDEX_BLOCK
// TRAILER

#[derive(Serialize, Deserialize, Debug)]
struct Trailer {
    start_of_meta_block: Offset,
    version: u8,
    stanza: Vec<u8>,
}

impl Trailer {
    fn new(start_of_meta_block: Offset) -> Trailer {
        Trailer {
            start_of_meta_block,
            version: 0x1,
            stanza: STANZA.as_bytes().to_vec(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Meta {
    data_size: usize,
    data_block_count: usize,
    index_size: usize,
}

pub struct Writer {
    file: io::BufWriter<fs::File>,
    data_bytes_written: usize,
    data_count: usize,
    index: Vec<(Key, usize)>,
    path: path::PathBuf,
}

impl Writer {
    pub fn create(path: &path::Path) -> Result<Self> {
        let file = io::BufWriter::new(OpenOptions::new().create(true).write(true).open(path)?);

        Ok(Writer {
            file,
            data_bytes_written: 0,
            data_count: 0,
            index: Vec::new(),
            path: path.to_owned(),
        })
    }

    pub fn add_data(&mut self, k: Key, v: Value) -> Result<()> {
        println!("appending data to SSTable");

        let pos = self.data_bytes_written;
        self.data_bytes_written += binio::write_frame(&mut self.file, &k)?;
        self.data_bytes_written += binio::write_frame(&mut self.file, &v)?;
        self.data_count += 1;
        self.index.push((k, pos));

        Ok(())
    }

    pub fn finish(mut self) -> Result<Slab> {
        let end_of_data = self.data_bytes_written;
        let meta = Meta {
            data_block_count: self.data_count,
            data_size: end_of_data,
            index_size: self.index.len(),
        };

        println!("writing meta data: {:?}", meta);

        let index_offset = end_of_data + binio::write_data(&mut self.file, &meta)?;
        let trailer_offset = index_offset + binio::write_data(&mut self.file, &self.index)?;
        let trailer = Trailer::new(end_of_data);

        binio::write_data(&mut self.file, &trailer)?;
        binio::write_u64(&mut self.file, trailer_offset as u64)?;
        self.file.flush()?;

        println!("meta data written");
        if (self.index.is_empty()) {
            return Err(Error::EmptyTable);
        }

        Ok(Slab {
            // TODO: add actual level
            level: 0,
            path: self.path,
            min_key: self.index[0].0.clone(),
            max_key: self.index.last().unwrap().0.clone(),
        })
    }
}

type ReaderStorage = io::BufReader<fs::File>;

pub struct Reader {
    file: ReaderStorage,
    meta: Meta,
}

impl Reader {
    pub fn open(path: &path::Path) -> Result<Self> {
        let mut file = io::BufReader::new(OpenOptions::new().read(true).open(path)?);
        let meta = Reader::read_meta(&mut file)?;
        file.seek(SeekFrom::Start(0))?;

        Ok(Reader { file, meta })
    }

    fn read_meta(file: &mut ReaderStorage) -> Result<Meta> {
        file.seek(SeekFrom::End(-8))?;
        let trailer_offset = binio::read_u64(file)?;

        file.seek(SeekFrom::Start(trailer_offset))?;
        let trailer: Trailer = binio::read_data_owned(file)?;

        file.seek(SeekFrom::Start(trailer.start_of_meta_block as u64))?;
        let meta: Meta = binio::read_data_owned(file)?;
        Ok(meta)
    }
}
