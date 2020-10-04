use std::io::{Read, Write};

use crate::read_util::{ByteOrder, Readable, Writable};
use crate::Result;

pub struct QueryFontRequest;

impl Readable for QueryFontRequest {
    fn read(_stream: &mut impl Read, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for QueryFontRequest{
    fn write(_stream: &mut impl Write, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}

pub struct QueryFontResponse;

impl Readable for QueryFontResponse {
    fn read(_stream: &mut impl Read, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for QueryFontResponse{
    fn write(_stream: &mut impl Write, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}
