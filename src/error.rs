use std::io;
use std::result;

use reqwest;
use reqwest::StatusCode;
use serde_json;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    HttpError(reqwest::Error),
    SerializeError(reqwest::Error),
    RequestError(reqwest::Error),
    InvalidJsonError(serde_json::error::Error),
    /// Contains the status code of the request
    RequestFailed(StatusCode),
    /// Indicates the JSON couldn't be found
    JsonNotFound,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        match err {
            reqwest::Error::Http(..) => Error::HttpError(err),
            reqwest::Error::Serialize(..) => Error::SerializeError(err),
            _ => Error::RequestError(err),
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::InvalidJsonError(err)
    }
}
