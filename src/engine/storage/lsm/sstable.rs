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
    SealedTableError,
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
    pub level: Level,
    pub max_key: Key,
    pub min_key: Key,
    pub path: path::PathBuf,
}

pub struct SSTable {
    // TODO: think about using a trie instead?
    index: HashMap<Key, Offset>,
    path: path::PathBuf,
    reader: Reader,
}

impl SSTable {
    pub fn get(&self, k: &Key) -> Result<Option<&Value>> {
        let offset = self.index.get(k);
        Ok(None)
    }

    pub fn open(path: &path::Path) -> Result<SSTable> {
        let mut reader = Reader::open(path)?;
        let mut index: HashMap<Key, Offset> = HashMap::new();
        reader.read_index_into(&mut index)?;

        Ok(SSTable {
            index,
            path: path.to_path_buf(),
            reader,
        })
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
    meta_offset: Offset,
    index_offset: Offset,
    version: u8,
    stanza: Vec<u8>,
}

impl Trailer {
    fn new(meta_offset: Offset, index_offset: Offset) -> Trailer {
        Trailer {
            meta_offset,
            index_offset,
            version: 0x1,
            stanza: STANZA.as_bytes().to_vec(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub data_size: usize,
    pub data_block_count: usize,
    pub index_size: usize,
}

pub struct Writer {
    file: io::BufWriter<fs::File>,
    data_bytes_written: usize,
    data_count: usize,
    index: Vec<(Key, usize)>,
    path: path::PathBuf,
    sealed: bool,
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
            sealed: false,
        })
    }

    pub fn add_data(&mut self, k: Key, v: Value) -> Result<()> {
        let pos = self.data_bytes_written;
        self.data_bytes_written += binio::write_frame(&mut self.file, &k)?;
        self.data_bytes_written += binio::write_frame(&mut self.file, &v)?;
        self.index.push((k, pos));
        self.data_count += 1;

        Ok(())
    }

    pub fn finish(&mut self) -> Result<Slab> {
        if self.sealed {
            return Err(Error::SealedTableError);
        }

        let meta_offset = self.write_meta()?;
        let index_offset = self.write_index()?;
        let _trailer_offset = self.write_trailer(meta_offset, index_offset)?;
        self.file.flush()?;
        self.sealed = true;

        let idx = &self.index;
        let (min_key, _) = idx.first().unwrap();
        let (max_key, _) = idx.last().unwrap();

        trace!(target: "SSTable::Writer", "sstable finished and sealed {:#?}", self.path);

        Ok(Slab {
            level: 0,
            path: self.path.to_owned(),
            min_key: min_key.to_vec(),
            max_key: max_key.to_vec(),
        })
    }

    fn write_meta(&mut self) -> Result<Offset> {
        let meta_offset = self.pos()?;
        let meta = Meta {
            data_block_count: self.data_count,
            data_size: self.data_bytes_written,
            index_size: self.index.len(),
        };

        trace!(target: "SSTable::Writer", "writing meta data: {:?} at offset: {}", meta, meta_offset);

        binio::write_data(&mut self.file, &meta)?;
        Ok(meta_offset)
    }

    fn write_trailer(&mut self, meta_offset: Offset, index_offset: Offset) -> Result<Offset> {
        let trailer_offset = self.pos()?;
        let trailer = Trailer::new(meta_offset, index_offset);

        trace!(target: "SSTable::Writer", "writing trailer: {:?} at offset: {}", trailer, trailer_offset);

        binio::write_data(&mut self.file, &trailer)?;
        binio::write_u64(&mut self.file, trailer_offset as u64)?;

        Ok(trailer_offset)
    }

    fn write_index(&mut self) -> Result<Offset> {
        let index_offset = self.pos()?;

        trace!(target: "SSTable::Writer", "writing index of size {} at offset: {}", &self.index.len(), index_offset);

        for (key, offset) in &self.index {
            binio::write_data(&mut self.file, key)?;
            binio::write_data(&mut self.file, *offset)?;
        }

        Ok(index_offset as Offset)
    }

    fn pos(&mut self) -> Result<Offset> {
        let pos = self.file.seek(SeekFrom::Current(0))?;
        Ok(pos as Offset)
    }
}

// Reader

type ReaderStorage = io::BufReader<fs::File>;

pub struct Reader {
    file: ReaderStorage,
    meta: Meta,
    trailer: Trailer,
}

impl Reader {
    pub fn open(path: &path::Path) -> Result<Self> {
        let mut file = io::BufReader::new(OpenOptions::new().read(true).open(path)?);
        let (meta, trailer) = Reader::read_control_data(&mut file)?;

        Ok(Reader {
            file,
            meta,
            trailer,
        })
    }

    pub fn read_record(&mut self, offset: Offset) -> Result<Value> {
        self.file.seek(SeekFrom::Start(offset as u64))?;
        let value = binio::read_data_owned(&mut self.file)?;
        Ok(value)
    }

    pub fn read_index_into(&mut self, index: &mut HashMap<Key, Offset>) -> Result<()> {
        self.file
            .seek(SeekFrom::Start(self.trailer.index_offset as u64))?;

        for _ in 0..self.meta.index_size {
            index.insert(
                binio::read_data_owned(&mut self.file)?,
                binio::read_data_owned(&mut self.file)?,
            );
        }

        Ok(())
    }

    fn read_control_data(file: &mut ReaderStorage) -> Result<(Meta, Trailer)> {
        file.seek(SeekFrom::End(-8))?;
        let trailer_offset = binio::read_u64(file)?;
        file.seek(SeekFrom::Start(trailer_offset as u64))?;

        let trailer: Trailer = binio::read_data_owned(file)?;
        trace!(target: "SSTable::Reader", "read trailer {:#?} at: {}", trailer,  trailer_offset);

        file.seek(SeekFrom::Start(trailer.meta_offset as u64))?;
        let meta: Meta = binio::read_data_owned(file)?;
        trace!(target: "SSTable::Reader", "read meta {:#?} at: {}", meta, trailer.meta_offset);

        Ok((meta, trailer))
    }
}
