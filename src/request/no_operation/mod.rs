use std::io::{Read, Write};

use crate::read_util::{ByteOrder, Readable, Writable};
use crate::Result;

#[derive(Clone, Debug, PartialEq)]
pub struct NoOperationRequest;

impl Readable for NoOperationRequest {
    fn read(_stream: &mut std::io::BufReader<impl Read>, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for NoOperationRequest{
    fn write(_stream: &mut std::io::BufWriter<impl Write>, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NoOperationResponse;

impl Readable for NoOperationResponse {
    fn read(_stream: &mut std::io::BufReader<impl Read>, _order: &ByteOrder) -> Result<Self> {
        unimplemented!()
    }
}

impl Writable for NoOperationResponse{
    fn write(_stream: &mut std::io::BufWriter<impl Write>, _data: Self, _order: &ByteOrder) -> Result<()> {
        unimplemented!()
    }
}
