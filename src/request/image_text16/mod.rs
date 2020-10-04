use std::io::{Read, Write};

use crate::read_util::{ByteOrder, Readable, Writable};
use crate::Result;

pub struct ImageText16Request;

impl Readable for ImageText16Request {
    fn read(_stream: &mut impl Read, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for ImageText16Request{
    fn write(_stream: &mut impl Write, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}

pub struct ImageText16Response;

impl Readable for ImageText16Response {
    fn read(_stream: &mut impl Read, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for ImageText16Response{
    fn write(_stream: &mut impl Write, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}
