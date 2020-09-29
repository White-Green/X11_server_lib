use std::io::{Read, Write};

use crate::{Error, Result};
use crate::read_util::{ByteOrder, read_specified_length};

mod test;

#[derive(Debug, Clone, PartialEq)]
pub struct ConnectionSetupInformation {
    protocol_major_version: u16,
    protocol_minor_version: u16,
    authorization_protocol_name: String,
    authorization_protocol_data: String,
}

pub fn read_setup(stream: &mut impl Read, buffer: &mut [u8]) -> Result<(ByteOrder, ConnectionSetupInformation)> {
    assert!(buffer.len() >= 10);
    read_specified_length(stream, buffer, 2)?;
    let order = match buffer[0] {
        0o102 => { ByteOrder::MSBFirst }
        0o154 => { ByteOrder::LSBFirst }
        _ => { return Err(Error::InvalidValue("byte order")); }
    };
    read_specified_length(stream, buffer, 10)?;
    let protocol_major_version = order.decode(&buffer[0..2]);
    let protocol_minor_version = order.decode(&buffer[2..4]);
    let authorization_protocol_name_length: u16 = order.decode(&buffer[4..6]);
    let authorization_protocol_data_length: u16 = order.decode(&buffer[6..8]);
    let name_length = authorization_protocol_name_length as usize;
    let data_length = authorization_protocol_data_length as usize;

    let mut name_total_length = (((-1isize ^ 3) as usize) & name_length) + ((name_length << 1 | name_length << 2) & 4);
    let mut name = Vec::with_capacity(name_total_length);
    while name_total_length > 0 {
        let read = read_specified_length(stream, buffer, name_total_length)?;
        name.extend_from_slice(&buffer[..read]);
        name_total_length -= read;
    }
    let authorization_protocol_name = std::str::from_utf8(&name[..name_length])
        .map_err(|e| Error::StringError(e))?
        .to_string();

    let mut data_total_length = (((-1isize ^ 3) as usize) & data_length) + ((data_length << 1 | data_length << 2) & 4);
    let mut data = Vec::with_capacity(data_total_length);
    while data_total_length > 0 {
        let read = read_specified_length(stream, buffer, data_total_length)?;
        data.extend_from_slice(&buffer[..read]);
        data_total_length -= read;
    }
    let authorization_protocol_data = std::str::from_utf8(&buffer[..data_length])
        .map_err(|e| Error::StringError(e))?
        .to_string();

    let information = ConnectionSetupInformation {
        protocol_major_version,
        protocol_minor_version,
        authorization_protocol_name,
        authorization_protocol_data,
    };

    Ok((order, information))
}

pub fn write_setup(stream: &mut impl Write, buffer: &mut [u8], order: &ByteOrder, info: ConnectionSetupInformation) -> Result<()> {
    assert!(buffer.len() >= 12);
    buffer[0] =
        match order {
            ByteOrder::MSBFirst => { 0o102 }
            ByteOrder::LSBFirst => { 0o154 }
        };
    order.encode(info.protocol_major_version, &mut buffer[2..4]);
    order.encode(info.protocol_minor_version, &mut buffer[4..6]);
    let name_len = info.authorization_protocol_name.as_bytes().len();
    order.encode(name_len as u16, &mut buffer[6..8]);
    let data_len = info.authorization_protocol_data.as_bytes().len();
    order.encode(data_len as u16, &mut buffer[8..10]);
    stream.write(&buffer[..12]).map_err(|e| Error::IoError(e))?;
    stream.write(info.authorization_protocol_name.as_bytes()).map_err(|e| Error::IoError(e))?;
    stream.write(&buffer[..((!name_len).wrapping_add(1)) & 0b11]).map_err(|e| Error::IoError(e))?;
    stream.write(info.authorization_protocol_data.as_bytes()).map_err(|e| Error::IoError(e))?;
    stream.write(&buffer[..((!data_len).wrapping_add(1)) & 0b11]).map_err(|e| Error::IoError(e))?;
    Ok(())
}