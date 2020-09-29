use std::io::Read;

use crate::{Error, Result};

mod test;

pub(crate) fn read_specified_length(stream: &mut impl Read, buffer: &mut [u8], length: usize) -> Result<usize> {
    let length = length.min(buffer.len());
    let mut read_length = 0;
    while read_length < length {
        let i = stream.read(&mut buffer[read_length..length])
            .map_err(|e| Error::IoError(e))?;
        read_length += i;
    }
    Ok(length)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ByteOrder {
    MSBFirst,
    LSBFirst,
}

impl ByteOrder {
    pub fn encode<T>(&self, data: T, dest: &mut [u8]) where T: Encoding {
        data.encode(self, dest);
    }
    pub fn decode<T>(&self, data: &[u8]) -> T where T: Encoding {
        T::decode(self, data)
    }
}

pub trait Encoding {
    fn encode(&self, order: &ByteOrder, dest: &mut [u8]);
    fn decode(order: &ByteOrder, data: &[u8]) -> Self;
}

impl Encoding for u8 {
    fn encode(&self, _order: &ByteOrder, dest: &mut [u8]) {
        assert!(dest.len() >= 1);
        dest[0] = *self;
    }

    fn decode(_order: &ByteOrder, data: &[u8]) -> Self {
        assert!(data.len() >= 1);
        data[0]
    }
}

impl Encoding for u16 {
    fn encode(&self, order: &ByteOrder, dest: &mut [u8]) {
        assert!(dest.len() >= 2);
        match order {
            ByteOrder::MSBFirst => {
                dest[0] = (self >> 8 & 255) as u8;
                dest[1] = (self >> 0 & 255) as u8;
            }
            ByteOrder::LSBFirst => {
                dest[0] = (self >> 0 & 255) as u8;
                dest[1] = (self >> 8 & 255) as u8;
            }
        }
    }

    fn decode(order: &ByteOrder, data: &[u8]) -> Self {
        assert!(data.len() >= 2);
        match order {
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

impl Encoding for u32 {
    fn encode(&self, order: &ByteOrder, dest: &mut [u8]) {
        assert!(dest.len() >= 4);
        match order {
            ByteOrder::MSBFirst => {
                dest[0] = (self >> 24 & 255) as u8;
                dest[1] = (self >> 16 & 255) as u8;
                dest[2] = (self >> 08 & 255) as u8;
                dest[3] = (self >> 00 & 255) as u8;
            }
            ByteOrder::LSBFirst => {
                dest[0] = (self >> 00 & 255) as u8;
                dest[1] = (self >> 08 & 255) as u8;
                dest[2] = (self >> 16 & 255) as u8;
                dest[3] = (self >> 24 & 255) as u8;
            }
        }
    }

    fn decode(order: &ByteOrder, data: &[u8]) -> Self {
        assert!(data.len() >= 4);
        match order {
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

impl Encoding for i8 {
    fn encode(&self, _order: &ByteOrder, dest: &mut [u8]) {
        assert!(dest.len() >= 1);
        dest[0] = *self as u8;
    }

    fn decode(_order: &ByteOrder, data: &[u8]) -> Self {
        assert!(data.len() >= 1);
        data[0] as i8
    }
}

impl Encoding for i16 {
    fn encode(&self, order: &ByteOrder, dest: &mut [u8]) {
        assert!(dest.len() >= 2);
        match order {
            ByteOrder::MSBFirst => {
                dest[0] = (self >> 8 & 255) as u8;
                dest[1] = (self >> 0 & 255) as u8;
            }
            ByteOrder::LSBFirst => {
                dest[0] = (self >> 0 & 255) as u8;
                dest[1] = (self >> 8 & 255) as u8;
            }
        }
    }

    fn decode(order: &ByteOrder, data: &[u8]) -> Self {
        assert!(data.len() >= 2);
        match order {
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

impl Encoding for i32 {
    fn encode(&self, order: &ByteOrder, dest: &mut [u8]) {
        assert!(dest.len() >= 4);
        match order {
            ByteOrder::MSBFirst => {
                dest[0] = (self >> 24 & 255) as u8;
                dest[1] = (self >> 16 & 255) as u8;
                dest[2] = (self >> 08 & 255) as u8;
                dest[3] = (self >> 00 & 255) as u8;
            }
            ByteOrder::LSBFirst => {
                dest[0] = (self >> 00 & 255) as u8;
                dest[1] = (self >> 08 & 255) as u8;
                dest[2] = (self >> 16 & 255) as u8;
                dest[3] = (self >> 24 & 255) as u8;
            }
        }
    }

    fn decode(order: &ByteOrder, data: &[u8]) -> Self {
        assert!(data.len() >= 4);
        match order {
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
