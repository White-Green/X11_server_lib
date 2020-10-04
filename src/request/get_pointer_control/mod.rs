use std::io::{Read, Write};

use crate::read_util::{ByteOrder, Readable, Writable};
use crate::Result;

#[derive(Clone, Debug, PartialEq)]
pub struct GetPointerControlRequest;

impl Readable for GetPointerControlRequest {
    fn read(_stream: &mut std::io::BufReader<impl Read>, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for GetPointerControlRequest{
    fn write(_stream: &mut std::io::BufWriter<impl Write>, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GetPointerControlResponse;

impl Readable for GetPointerControlResponse {
    fn read(_stream: &mut std::io::BufReader<impl Read>, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for GetPointerControlResponse{
    fn write(_stream: &mut std::io::BufWriter<impl Write>, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}
