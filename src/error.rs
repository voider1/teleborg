use std::io;
use std::result;

use reqwest;
use reqwest::StatusCode;
use serde_json;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    RequestError(reqwest::Error),
    InvalidJSONError(serde_json::error::Error),
    /// Contains the status code of the request
    RequestFailed(StatusCode),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IOError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::RequestError(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::InvalidJSONError(err)
    }
}
