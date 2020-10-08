use std::io::{Read, Write};

use crate::read_util::{ByteOrder, read_specified_length, Readable, ReadableRead, Writable, WritableWrite};
use crate::Result;

mod test;

#[derive(Clone, Debug, PartialEq)]
pub struct GetPropertyRequest {
    delete: bool,
    window: u32,
    property: u32,
    type_: Option<u32>,
    long_offset: u32,
    long_length: u32,
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
pub struct GetPropertyResponse;

impl Readable for GetPropertyResponse {
    fn read(_stream: &mut std::io::BufReader<impl Read>, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for GetPropertyResponse {
    fn write(_stream: &mut std::io::BufWriter<impl Write>, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}
