use std::io::Read;

use crate::{Error, Result};
use crate::read_util::{ByteOrder, Collect, read_specified_length};

#[derive(Debug, Clone)]
pub struct ConnectionSetupInformation {
    protocol_major_version: u16,
    protocol_minor_version: u16,
    authorization_protocol_name: String,
    authorization_protocol_data: String,
}

pub fn read_setup(stream: &mut impl Read, buffer: &mut [u8]) -> Result<(ByteOrder, ConnectionSetupInformation)> {
    read_specified_length(stream, buffer, 2)?;
    let order = match buffer[0] {
        0o102 => { ByteOrder::MSBFirst }
        0o154 => { ByteOrder::LSBFirst }
        _ => { return Err(Error::InvalidValue("byte order")); }
    };
    read_specified_length(stream, buffer, 10)?;
    let protocol_major_version = order.collect(&buffer[0..2]);
    let protocol_minor_version = order.collect(&buffer[2..4]);
    let authorization_protocol_name_length: u16 = order.collect(&buffer[4..6]);
    let authorization_protocol_data_length: u16 = order.collect(&buffer[6..8]);
    let name_length = authorization_protocol_name_length as usize;
    let data_length = authorization_protocol_data_length as usize;

    let name_total_length = (((-1isize ^ 3) as usize) & name_length) + ((name_length << 1 | name_length << 2) & 4);
    read_specified_length(stream, buffer, name_total_length)?;
    let authorization_protocol_name = std::str::from_utf8(&buffer[..name_length])
        .map_err(|e| Error::StringError(e))?
        .to_string();

    let data_total_length = (((-1isize ^ 3) as usize) & data_length) + ((data_length << 1 | data_length << 2) & 4);
    read_specified_length(stream, buffer, data_total_length)?;
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