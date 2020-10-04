use std::io::{Read, Write};

use crate::read_util::{ByteOrder, Readable, Writable};
use crate::Result;

pub struct UnmapSubwindowsRequest;

impl Readable for UnmapSubwindowsRequest {
    fn read(_stream: &mut impl Read, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for UnmapSubwindowsRequest{
    fn write(_stream: &mut impl Write, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}

pub struct UnmapSubwindowsResponse;

impl Readable for UnmapSubwindowsResponse {
    fn read(_stream: &mut impl Read, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for UnmapSubwindowsResponse{
    fn write(_stream: &mut impl Write, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}
