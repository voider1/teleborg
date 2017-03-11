use std::{io, result};

use reqwest;
use reqwest::StatusCode;
use serde_json;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    HttpError(reqwest::Error),
    SerializeError(reqwest::Error),
    RequestError(reqwest::Error),
    /// Contains the status code of the request
    RequestFailed(StatusCode),
    /// Indicates the JSON couldn't be found
    JsonNotFound,
    /// Anything that can go wrong with the JSON
    JsonError(serde_json::Error),
    TelegramError(String),
}

pub fn check_for_error(json: serde_json::Value) -> Result<serde_json::Value> {
    let status = json.get("ok").ok_or(Error::JsonNotFound)?.as_bool().unwrap();

    if !status {
        let description =
            json.get("description").ok_or(Error::JsonNotFound)?.as_str().unwrap().to_string();
        Err(Error::TelegramError(description))
    } else {
        Ok(json)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
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
    fn from(err: serde_json::Error) -> Error {
        Error::JsonError(err)
    }
}
