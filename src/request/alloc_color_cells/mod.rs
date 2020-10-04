use std::io::{Read, Write};

use crate::read_util::{ByteOrder, Readable, Writable};
use crate::Result;

#[derive(Clone, Debug, PartialEq)]
pub struct AllocColorCellsRequest;

impl Readable for AllocColorCellsRequest {
    fn read(_stream: &mut std::io::BufReader<impl Read>, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for AllocColorCellsRequest{
    fn write(_stream: &mut std::io::BufWriter<impl Write>, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AllocColorCellsResponse;

impl Readable for AllocColorCellsResponse {
    fn read(_stream: &mut std::io::BufReader<impl Read>, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for AllocColorCellsResponse{
    fn write(_stream: &mut std::io::BufWriter<impl Write>, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}
