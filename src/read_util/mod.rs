use std::io::Read;

use crate::{Error, Result};

pub(crate) fn read_specified_length(stream: &mut impl Read, buffer: &mut [u8], mut length: usize) -> Result<()> {
    let mut read_length = 0;
    while read_length < length {
        read_length += stream.read(&mut buffer[read_length..length]).map_err(|e| Error::IoError(e))?;
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub enum ByteOrder {
    MSBFirst,
    LSBFirst,
}

pub(crate) trait Collect<T> { fn collect(&self, data: &[u8]) -> T; }

impl Collect<u8> for ByteOrder {
    fn collect(&self, data: &[u8]) -> u8 {
        assert_eq!(data.len(), 1);
        data[0]
    }
}

impl Collect<u16> for ByteOrder {
    fn collect(&self, data: &[u8]) -> u16 {
        assert_eq!(data.len(), 2);
        let mut result = 0;
        match self {
            ByteOrder::MSBFirst => {
                for &d in data {
                    result <<= 8;
                    result |= d as u16;
                }
            }
            ByteOrder::LSBFirst => {
                for i in 0..data.len() {
                    result |= (data[i] as u16) << i * 8;
                }
            }
        }
        result
    }
}

impl Collect<u32> for ByteOrder {
    fn collect(&self, data: &[u8]) -> u32 {
        assert_eq!(data.len(), 4);
        let mut result = 0;
        match self {
            ByteOrder::MSBFirst => {
                for &d in data {
                    result <<= 8;
                    result |= d as u32;
                }
            }
            ByteOrder::LSBFirst => {
                for i in 0..data.len() {
                    result |= (data[i] as u32) << i * 8;
                }
            }
        }
        result
    }
}

impl Collect<i8> for ByteOrder {
    fn collect(&self, data: &[u8]) -> i8 {
        assert_eq!(data.len(), 1);
        data[0] as i8
    }
}

impl Collect<i16> for ByteOrder {
    fn collect(&self, data: &[u8]) -> i16 {
        assert_eq!(data.len(), 2);
        let mut result = 0;
        match self {
            ByteOrder::MSBFirst => {
                for &d in data {
                    result <<= 8;
                    result |= d as i16;
                }
            }
            ByteOrder::LSBFirst => {
                for i in 0..data.len() {
                    result |= (data[i] as i16) << i * 8;
                }
            }
        }
        result
    }
}

impl Collect<i32> for ByteOrder {
    fn collect(&self, data: &[u8]) -> i32 {
        assert_eq!(data.len(), 4);
        let mut result = 0;
        match self {
            ByteOrder::MSBFirst => {
                for &d in data {
                    result <<= 8;
                    result |= d as i32;
                }
            }
            ByteOrder::LSBFirst => {
                for i in 0..data.len() {
                    result |= (data[i] as i32) << i * 8;
                }
            }
        }
        result
    }
}
