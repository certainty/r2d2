use byteorder::LittleEndian;
use byteorder::{ReadBytesExt, WriteBytesExt};
use log::{error, trace};
use std::io;
use std::io::{BufReader, BufWriter, Read};
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    SerializationError,
    IoError(io::ErrorKind),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e.kind())
    }
}

pub fn write_data<W, D>(w: &mut W, data: D) -> Result<usize>
where
    W: io::Write,
    D: serde::Serialize,
{
    let serialized = bincode::serialize(&data).map_err(|e| {
        error!(target: "WAL", "serialization of data frame failed: {:?}", e.as_ref());
        Error::SerializationError
    })?;

    write_frame(w, &serialized)
}

// TODO: make sure we really need the owned variant here
// could be a problem since data may be copied
pub fn read_data<R, D>(r: &mut R) -> Result<D>
where
    R: io::Read,
    D: serde::de::DeserializeOwned,
{
    let mut buf = Vec::new();
    read_frame(r, &mut buf)?;
    let value = bincode::deserialize(buf.as_slice()).map_err(|e| {
        error!(target: "WAL", "deserialization of data frame failed: {:?}", e.as_ref());
        Error::SerializationError
    })?;
    Ok(value)
}

pub fn read_frame<R>(reader: &mut R, buf: &mut Vec<u8>) -> Result<usize>
where
    R: io::Read,
{
    let size = reader.read_u64::<LittleEndian>()?;
    reader.take(size).read_to_end(buf)?;
    Ok(size as usize)
}

pub fn write_frame<W>(writer: &mut W, data: &[u8]) -> Result<usize>
where
    W: io::Write,
{
    writer.write_u64::<LittleEndian>(data.len() as u64)?;
    writer.write_all(data)?;
    writer.flush()?;
    Ok(data.len())
}
