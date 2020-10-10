use std::io::{Read, Write};

use crate::{Error, Result};
use crate::read_util::{ByteOrder, read_specified_length, Readable, ReadableRead, Writable, WritableWrite};

mod test;

#[derive(Clone, Debug, PartialEq)]
pub struct GetPropertyRequest {
    pub delete: bool,
    pub window: u32,
    pub property: u32,
    pub type_: Option<u32>,
    pub long_offset: u32,
    pub long_length: u32,
}

impl Readable for GetPropertyRequest {
    fn read(stream: &mut std::io::BufReader<impl Read>, order: &ByteOrder) -> Result<Self> {
        let delete = stream.read_value(order)?;
        read_specified_length(stream, &mut [0; 2], 2)?;
        let window = stream.read_value(order)?;
        let property = stream.read_value(order)?;
        let type_ = match stream.read_value(order)? {
            0 => None,
            other => Some(other)
        };
        let long_offset = stream.read_value(order)?;
        let long_length = stream.read_value(order)?;
        Ok(GetPropertyRequest {
            delete,
            window,
            property,
            type_,
            long_offset,
            long_length,
        })
    }
}

impl Writable for GetPropertyRequest {
    fn write(stream: &mut std::io::BufWriter<impl Write>, data: Self, order: &ByteOrder) -> Result<()> {
        stream.write_value::<u8>(20, order)?;
        stream.write_value(data.delete, order)?;
        stream.write_value::<u16>(6, order)?;
        stream.write_value(data.window, order)?;
        stream.write_value(data.property, order)?;
        stream.write_value(data.type_.unwrap_or(0), order)?;
        stream.write_value(data.long_offset, order)?;
        stream.write_value(data.long_length, order)?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GetPropertyResponse {
    pub format: u8,
    pub sequence_number: u16,
    pub type_: Option<u32>,
    pub bytes_after: u32,
    pub length_of_value_in_format_units: u32,
    pub value: Vec<u8>,
}

impl Readable for GetPropertyResponse {
    fn read(stream: &mut std::io::BufReader<impl Read>, order: &ByteOrder) -> Result<Self> {
        read_specified_length(stream, &mut [0], 1)?;
        let format = stream.read_value(order)?;
        let sequence_number = stream.read_value(order)?;
        let len = (stream.read_value::<u32>(order)? as usize) << 2;
        let type_ = match stream.read_value(order)? {
            0 => None,
            other => Some(other),
        };
        let bytes_after = stream.read_value(order)?;
        let length_of_value_in_format_units = stream.read_value(order)?;
        read_specified_length(stream, &mut [0; 12], 12)?;
        let value_len =
            if (length_of_value_in_format_units << 2) as usize <= len {
                (length_of_value_in_format_units << 2) as usize
            } else if (length_of_value_in_format_units << 1) as usize <= len {
                (length_of_value_in_format_units << 1) as usize
            } else {
                length_of_value_in_format_units as usize
            };
        let mut value = vec![0; len];
        read_specified_length(stream, &mut value, len)?;
        for _ in value_len..len {
            value.pop();
        }
        Ok(GetPropertyResponse {
            format,
            sequence_number,
            type_,
            bytes_after,
            length_of_value_in_format_units,
            value,
        })
    }
}

impl Writable for GetPropertyResponse {
    fn write(stream: &mut std::io::BufWriter<impl Write>, data: Self, order: &ByteOrder) -> Result<()> {
        stream.write_value::<u8>(1, order)?;
        stream.write_value(data.format, order)?;
        stream.write_value(data.sequence_number, order)?;
        stream.write_value::<u32>((((data.value.len() + 3) & !3) >> 2) as u32, order)?;
        stream.write_value(data.type_.unwrap_or(0), order)?;
        stream.write_value(data.bytes_after, order)?;
        stream.write_value(data.length_of_value_in_format_units, order)?;
        stream.write_all(&[0; 12]).map_err(|e| Error::IoError(e))?;
        stream.write_all(&data.value[..]).map_err(|e| Error::IoError(e))?;
        stream.write_all(&[0; 4][..(!data.value.len()).wrapping_add(1) & 3]).map_err(|e| Error::IoError(e))?;
        Ok(())
    }
}
