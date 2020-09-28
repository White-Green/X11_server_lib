use std::str::Utf8Error;

pub mod read_util;
pub mod setup;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    StringError(Utf8Error),
    InvalidValue(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;

