use std::io::{Read, Write};

use crate::read_util::{ByteOrder, Readable, Writable};
use crate::Result;

pub struct BellRequest;

impl Readable for BellRequest {
    fn read(_stream: &mut impl Read, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for BellRequest{
    fn write(_stream: &mut impl Write, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}

pub struct BellResponse;

impl Readable for BellResponse {
    fn read(_stream: &mut impl Read, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for BellResponse{
    fn write(_stream: &mut impl Write, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}
