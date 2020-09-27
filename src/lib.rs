use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;
use std::str::Utf8Error;

use crate::read_util::{ByteOrder, Collect, read_specified_length};

pub mod read_util;
pub mod setup;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    StringError(Utf8Error),
    InvalidValue(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;

