#![deny(unused_must_use)]

use std::str::Utf8Error;

pub mod read_util;
pub mod setup;
pub mod request;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    StringError(Utf8Error),
    InvalidValue(&'static str),
    UnknownError,
}

pub type Result<T> = std::result::Result<T, Error>;

