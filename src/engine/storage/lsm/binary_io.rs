//! Binary IO primitives
//!
//! This is an internal module that provides primitive operations to do binary IO
//! It provides methods to read and write length tagged data frames as well as a
//! generic capability to read and write serde serializable data.
use byteorder::LittleEndian;
use byteorder::{ReadBytesExt, WriteBytesExt};
use log::{error, trace};
use std::io;
use std::io::Read;

type Result<T> = std::result::Result<T, Error>;
pub const LENGTH_TAG_SIZE: i8 = 4;

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

pub fn read_data_size<R>(r: &mut R) -> Result<u32>
where
    R: io::Read,
{
    r.read_u32::<LittleEndian>()
        .map_err(|e| Error::IoError(e.kind()))
}

pub fn write_data_size<W>(w: &mut W, d: usize) -> Result<usize>
where
    W: io::Write,
{
    w.write_u32::<LittleEndian>(d as u32)?;
    Ok(4)
}

pub fn write_data<W, D>(w: &mut W, data: D) -> Result<usize>
where
    W: io::Write,
    D: serde::Serialize,
{
    let serialized = bincode::serialize(&data).map_err(|e| {
        error!("serialization of data frame failed: {:?}", e.as_ref());
        Error::SerializationError
    })?;

    write_frame(w, &serialized)
}

pub fn read_data_owned<R, D>(r: &mut R) -> Result<D>
where
    R: io::Read,
    D: serde::de::DeserializeOwned,
{
    let mut buf = Vec::new();
    let frame_size = read_frame(r, &mut buf)?;
    trace!("read data frame successfully. size: {} bytes", frame_size);
    let value = bincode::deserialize(buf.as_slice()).map_err(|e| {
        error!("deserialization of data frame failed: {:?}", e.as_ref());
        Error::SerializationError
    })?;
    Ok(value)
}

pub fn read_frame<R>(reader: &mut R, buf: &mut Vec<u8>) -> Result<usize>
where
    R: io::Read,
{
    trace!("read data size tag: {} bytes", LENGTH_TAG_SIZE);
    let size = read_data_size(reader)?;
    trace!("read data frame: {} bytes", size);
    reader.take(size as u64).read_to_end(buf)?;
    Ok((size as i32 + (LENGTH_TAG_SIZE as i32)) as usize)
}

pub fn write_frame<W>(writer: &mut W, data: &[u8]) -> Result<usize>
where
    W: io::Write,
{
    let size_bytes = write_data_size(writer, data.len())?;
    trace!("wrote data size tag: {} bytes", size_bytes);
    writer.write_all(data)?;
    trace!("wrote data frame: {} bytes", data.len());
    Ok(data.len() + size_bytes)
}
