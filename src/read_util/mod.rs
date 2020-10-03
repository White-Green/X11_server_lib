use std::io::{Read, Write};

use crate::{Error, Result};

mod test;

pub(crate) fn read_specified_length(stream: &mut impl Read, buffer: &mut [u8], length: usize) -> Result<usize> {
    let length = length.min(buffer.len());
    let mut read_length = 0;
    while read_length < length {
        let i = stream.read(&mut buffer[read_length..length])
            .map_err(|e| Error::IoError(e))?;
        if i == 0 {
            return Err(Error::UnknownError);
        }
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
    const SIZE: usize;
    fn encode(&self, order: &ByteOrder, dest: &mut [u8]);
    fn decode(order: &ByteOrder, data: &[u8]) -> Self;
}

impl Encoding for bool {
    const SIZE: usize = 1;
    fn encode(&self, _order: &ByteOrder, dest: &mut [u8]) {
        assert!(dest.len() >= 1);
        dest[0] = if *self { 1 } else { 0 };
    }

    fn decode(_order: &ByteOrder, data: &[u8]) -> Self {
        assert!(data.len() >= 1);
        data[0] != 0
    }
}

impl Encoding for u8 {
    const SIZE: usize = 1;
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
    const SIZE: usize = 2;
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
    const SIZE: usize = 4;
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
    const SIZE: usize = 1;
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
    const SIZE: usize = 2;
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
    const SIZE: usize = 4;
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

pub trait Readable: Sized {
    fn read(stream: &mut impl Read, order: &ByteOrder) -> Result<Self>;
}

pub trait Writable: Sized {
    fn write(stream: &mut impl Write, data: Self, order: &ByteOrder) -> Result<()>;
}

impl<V: Sized + Encoding> Readable for V {
    fn read(stream: &mut impl Read, order: &ByteOrder) -> Result<Self> {
        let mut buffer = vec![0; V::SIZE];
        read_specified_length(stream, &mut buffer[..], V::SIZE)?;
        Ok(V::decode(order, &buffer[..]))
    }
}

impl<V: Sized + Encoding> Writable for V {
    fn write(stream: &mut impl Write, data: V, order: &ByteOrder) -> Result<()> {
        let mut buffer = vec![0; V::SIZE];
        data.encode(order, &mut buffer[..]);
        stream.write(&buffer[..]).map_err(|e| Error::IoError(e))?;
        Ok(())
    }
}

pub trait ReadableRead {
    fn read_value<T: Readable>(&mut self, order: &ByteOrder) -> Result<T>;
}

pub trait WritableWrite {
    fn write_value<T: Writable>(&mut self, data: T, order: &ByteOrder) -> Result<()>;
}

impl<S: Read> ReadableRead for S {
    fn read_value<T: Readable>(&mut self, order: &ByteOrder) -> Result<T> {
        T::read(self, order)
    }
}

impl<S: Write> WritableWrite for S {
    fn write_value<T: Writable>(&mut self, data: T, order: &ByteOrder) -> Result<()> {
        T::write(self, data, order)
    }
}