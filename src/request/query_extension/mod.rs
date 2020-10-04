use std::io::{Read, Write};

use crate::{Error, Result};
use crate::read_util::{ByteOrder, read_specified_length, Readable, ReadableRead, Writable, WritableWrite};

pub struct QueryExtensionRequest {
    pub name: String,
}

impl Readable for QueryExtensionRequest {
    fn read(stream: &mut std::io::BufReader<impl Read>, order: &ByteOrder) -> Result<Self> {
        read_specified_length(stream, &mut [0; 1], 1)?;
        let length = stream.read_value::<u16>(order)? as usize;
        let length_of_name = stream.read_value::<u16>(order)? as usize;
        let buffer_length = (length - 2) << 2;
        let mut buffer = vec![0; buffer_length];
        read_specified_length(stream, &mut buffer[..], buffer_length)?;
        Ok(QueryExtensionRequest { name: String::from(std::str::from_utf8(&buffer[..length_of_name]).map_err(|e| Error::StringError(e))?) })
    }
}

impl Writable for QueryExtensionRequest {
    fn write(stream: &mut std::io::BufWriter<impl Write>, data: Self, order: &ByteOrder) -> Result<()> {
        stream.write_value::<u8>(98, order)?;
        stream.write_all(&[0]).map_err(|e| Error::IoError(e))?;
        let len = 2 + ((data.name.as_bytes().len() + 3) >> 2);
        stream.write_value(len as u16, order)?;
        stream.write_all(&[0; 2]).map_err(|e| Error::IoError(e))?;
        stream.write_all(data.name.as_bytes()).map_err(|e| Error::IoError(e))?;
        let buf = vec![0; (!data.name.as_bytes().len()).wrapping_add(1) & 3];
        stream.write_all(&buf[..]).map_err(|e| Error::IoError(e))?;
        Ok(())
    }
}

pub struct QueryExtensionResponse {
    pub sequence_number: u16,
    pub major_opcode: u8,
    pub first_event: u8,
    pub first_error: u8,
}

impl Readable for QueryExtensionResponse {
    fn read(_stream: &mut std::io::BufReader<impl Read>, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for QueryExtensionResponse {
    fn write(_stream: &mut std::io::BufWriter<impl Write>, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}
