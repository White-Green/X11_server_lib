use std::io::{Read, Write};

use crate::read_util::{ByteOrder, Readable, Writable};
use crate::Result;

#[derive(Clone, Debug, PartialEq)]
pub struct UnmapWindowRequest;

impl Readable for UnmapWindowRequest {
    fn read(_stream: &mut std::io::BufReader<impl Read>, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for UnmapWindowRequest{
    fn write(_stream: &mut std::io::BufWriter<impl Write>, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnmapWindowResponse;

impl Readable for UnmapWindowResponse {
    fn read(_stream: &mut std::io::BufReader<impl Read>, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for UnmapWindowResponse{
    fn write(_stream: &mut std::io::BufWriter<impl Write>, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}
