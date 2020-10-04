use std::io::{Read, Write};

use crate::read_util::{ByteOrder, Readable, Writable};
use crate::Result;

#[derive(Clone, Debug, PartialEq)]
pub struct UnmapSubwindowsRequest;

impl Readable for UnmapSubwindowsRequest {
    fn read(_stream: &mut std::io::BufReader<impl Read>, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for UnmapSubwindowsRequest{
    fn write(_stream: &mut std::io::BufWriter<impl Write>, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnmapSubwindowsResponse;

impl Readable for UnmapSubwindowsResponse {
    fn read(_stream: &mut std::io::BufReader<impl Read>, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for UnmapSubwindowsResponse{
    fn write(_stream: &mut std::io::BufWriter<impl Write>, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}
