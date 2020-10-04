use std::io::{Read, Write};

use crate::read_util::{ByteOrder, Readable, Writable};
use crate::Result;

pub struct OpenFontRequest;

impl Readable for OpenFontRequest {
    fn read(_stream: &mut impl Read, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for OpenFontRequest{
    fn write(_stream: &mut impl Write, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}

pub struct OpenFontResponse;

impl Readable for OpenFontResponse {
    fn read(_stream: &mut impl Read, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for OpenFontResponse{
    fn write(_stream: &mut impl Write, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}
