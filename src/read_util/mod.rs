use std::io::Read;

use crate::{Error, Result};

mod test;

pub(crate) fn read_specified_length(stream: &mut impl Read, buffer: &mut [u8], length: usize) -> Result<()> {
    let mut read_length = 0;
    while read_length < length {
        let i = stream.read(&mut buffer[read_length..length])
            .map_err(|e| Error::IoError(e))?;
        read_length += i;
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub enum ByteOrder {
    MSBFirst,
    LSBFirst,
}

impl ByteOrder {
    fn encode<T>(&self, data: T, dest: &mut [u8]) where Self: Encoding<T> {
        <Self as Encoding<T>>::encode(self, data, dest);
    }
    fn decode<T>(&self, data: &[u8]) -> T where Self: Encoding<T> {
        <Self as Encoding<T>>::decode(self, data)
    }
}

pub(crate) trait Encoding<T> {
    fn encode(&self, data: T, dest: &mut [u8]);
    fn decode(&self, data: &[u8]) -> T;
}

impl Encoding<u8> for ByteOrder {
    fn encode(&self, data: u8, dest: &mut [u8]) {
        assert_eq!(dest.len(), 1);
        dest[0] = data;
    }

    fn decode(&self, data: &[u8]) -> u8 {
        assert_eq!(data.len(), 1);
        data[0]
    }
}

impl Encoding<u16> for ByteOrder {
    fn encode(&self, data: u16, dest: &mut [u8]) {
        assert_eq!(dest.len(), 2);
        match self {
            ByteOrder::MSBFirst => {
                dest[0] = (data >> 8 & 255) as u8;
                dest[1] = (data >> 0 & 255) as u8;
            }
            ByteOrder::LSBFirst => {
                dest[0] = (data >> 0 & 255) as u8;
                dest[1] = (data >> 8 & 255) as u8;
            }
        }
    }

    fn decode(&self, data: &[u8]) -> u16 {
        assert_eq!(data.len(), 2);
        match self {
            ByteOrder::MSBFirst => {
                (data[0] as u16) << 8 |
                    (data[1] as u16) << 0
            }
            ByteOrder::LSBFirst => {
                (data[1] as u16) << 8 |
                    (data[0] as u16)
            }
        }
    }
}

impl Encoding<u32> for ByteOrder {
    fn encode(&self, data: u32, dest: &mut [u8]) {
        assert_eq!(dest.len(), 4);
        match self {
            ByteOrder::MSBFirst => {
                dest[0] = (data >> 24 & 255) as u8;
                dest[1] = (data >> 16 & 255) as u8;
                dest[2] = (data >> 08 & 255) as u8;
                dest[3] = (data >> 00 & 255) as u8;
            }
            ByteOrder::LSBFirst => {
                dest[0] = (data >> 00 & 255) as u8;
                dest[1] = (data >> 08 & 255) as u8;
                dest[2] = (data >> 16 & 255) as u8;
                dest[3] = (data >> 24 & 255) as u8;
            }
        }
    }

    fn decode(&self, data: &[u8]) -> u32 {
        assert_eq!(data.len(), 4);
        match self {
            ByteOrder::MSBFirst => {
                (data[0] as u32) << 24 |
                    (data[1] as u32) << 16 |
                    (data[2] as u32) << 8 |
                    (data[3] as u32) << 0
            }
            ByteOrder::LSBFirst => {
                (data[0] as u32) << 00 |
                    (data[1] as u32) << 08 |
                    (data[2] as u32) << 16 |
                    (data[3] as u32) << 24
            }
        }
    }
}

impl Encoding<i8> for ByteOrder {
    fn encode(&self, data: i8, dest: &mut [u8]) {
        assert_eq!(dest.len(), 1);
        dest[0] = data as u8;
    }

    fn decode(&self, data: &[u8]) -> i8 {
        assert_eq!(data.len(), 1);
        data[0] as i8
    }
}

impl Encoding<i16> for ByteOrder {
    fn encode(&self, data: i16, dest: &mut [u8]) {
        assert_eq!(dest.len(), 2);
        match self {
            ByteOrder::MSBFirst => {
                dest[0] = (data >> 8 & 255) as u8;
                dest[1] = (data >> 0 & 255) as u8;
            }
            ByteOrder::LSBFirst => {
                dest[0] = (data >> 0 & 255) as u8;
                dest[1] = (data >> 8 & 255) as u8;
            }
        }
    }

    fn decode(&self, data: &[u8]) -> i16 {
        assert_eq!(data.len(), 2);
        match self {
            ByteOrder::MSBFirst => {
                (data[0] as i16) << 8 |
                    (data[1] as i16) << 0
            }
            ByteOrder::LSBFirst => {
                (data[1] as i16) << 8 |
                    (data[0] as i16)
            }
        }
    }
}

impl Encoding<i32> for ByteOrder {
    fn encode(&self, data: i32, dest: &mut [u8]) {
        assert_eq!(dest.len(), 4);
        match self {
            ByteOrder::MSBFirst => {
                dest[0] = (data >> 24 & 255) as u8;
                dest[1] = (data >> 16 & 255) as u8;
                dest[2] = (data >> 08 & 255) as u8;
                dest[3] = (data >> 00 & 255) as u8;
            }
            ByteOrder::LSBFirst => {
                dest[0] = (data >> 00 & 255) as u8;
                dest[1] = (data >> 08 & 255) as u8;
                dest[2] = (data >> 16 & 255) as u8;
                dest[3] = (data >> 24 & 255) as u8;
            }
        }
    }

    fn decode(&self, data: &[u8]) -> i32 {
        assert_eq!(data.len(), 4);
        match self {
            ByteOrder::MSBFirst => {
                (data[0] as i32) << 24 |
                    (data[1] as i32) << 16 |
                    (data[2] as i32) << 8 |
                    (data[3] as i32) << 0
            }
            ByteOrder::LSBFirst => {
                (data[0] as i32) << 00 |
                    (data[1] as i32) << 08 |
                    (data[2] as i32) << 16 |
                    (data[3] as i32) << 24
            }
        }
    }
}
