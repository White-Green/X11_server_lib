use std::io::{Read, Write};

use crate::{Error, Result};
use crate::read_util::{ByteOrder, read_specified_length, Readable, ReadableRead, Writable, WritableWrite};

mod test;

#[derive(Clone, Debug, PartialEq)]
pub struct InternAtomRequest {
    only_if_exists: bool,
    name: String,
}

impl Readable for InternAtomRequest {
    fn read(stream: &mut std::io::BufReader<impl Read>, order: &ByteOrder) -> Result<Self> {
        let only_if_exists = stream.read_value(order)?;
        let total_length = stream.read_value::<u16>(order)? as usize;
        let total_length = (total_length - 2) << 2;
        let name_len = stream.read_value::<u16>(order)? as usize;
        read_specified_length(stream, &mut [0; 2], 2)?;
        let mut buf = vec![0; total_length];
        read_specified_length(stream, &mut buf[..], total_length)?;
        let name = std::str::from_utf8(&buf[..name_len]).map_err(|e| Error::StringError(e))?;
        Ok(InternAtomRequest {
            only_if_exists,
            name: name.to_string(),
        })
    }
}

impl Writable for InternAtomRequest {
    fn write(stream: &mut std::io::BufWriter<impl Write>, data: Self, order: &ByteOrder) -> Result<()> {
        stream.write_value::<u8>(16, order)?;
        stream.write_value(data.only_if_exists, order)?;
        let name_len = data.name.as_bytes().len();
        stream.write_value((2 + ((name_len + 3) >> 2)) as u16, order)?;
        stream.write_value(name_len as u16, order)?;
        stream.write_all(&[0; 2]).map_err(|e| Error::IoError(e))?;
        stream.write_all(data.name.as_bytes()).map_err(|e| Error::IoError(e))?;
        stream.write_all(&[0; 4][..(!name_len).wrapping_add(1) & !3]).map_err(|e| Error::IoError(e))?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InternAtomResponse {
    sequence_number: u16,
    atom: Option<u32>,
}

impl Readable for InternAtomResponse {
    fn read(stream: &mut std::io::BufReader<impl Read>, order: &ByteOrder) -> Result<Self> {
        read_specified_length(stream, &mut [0; 2], 2)?;
        let sequence_number = stream.read_value(order)?;
        read_specified_length(stream, &mut [0; 4], 4)?;
        let atom = match stream.read_value(order)? {
            0 => None,
            other => Some(other),
        };
        read_specified_length(stream, &mut [0; 20], 20)?;
        Ok(InternAtomResponse {
            sequence_number,
            atom,
        })
    }
}

impl Writable for InternAtomResponse {
    fn write(stream: &mut std::io::BufWriter<impl Write>, data: Self, order: &ByteOrder) -> Result<()> {
        stream.write_value::<u8>(1, order)?;
        stream.write_all(&[0]).map_err(|e| Error::IoError(e))?;
        stream.write_value(data.sequence_number)?;
        stream.write_all(&[0; 4]).map_err(|e| Error::IoError(e))?;
        stream.write_value(data.atom.unwrap_or(0), order)?;
        stream.write_all(&[0; 20]).map_err(|e| Error::IoError(e))?;
        Ok(())
    }
}
