use std::{io, result};

use reqwest;
use serde_json;

pub type Result<T> = result::Result<T, Error>;

/// The Errors which may occur when using the Telegram bot API.
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    HttpError(String),
    SerializeError(String),
    RequestError(String),
    /// Indicates the JSON couldn't be found
    JsonNotFound,
    /// Anything that can go wrong with the JSON
    JsonError(serde_json::Error),
    TelegramError(String),
}

/// Check if there occured an error when querying the API.
pub fn check_for_error(json: serde_json::Value) -> Result<serde_json::Value> {
    let status = json.get("ok")
        .ok_or(Error::JsonNotFound)?
        .as_bool()
        .unwrap();

    if !status {
        let description = json.get("description")
            .ok_or(Error::JsonNotFound)?
            .as_str()
            .unwrap()
            .to_string();
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
        if err.is_http() {
            match err.url() {
                None => Error::HttpError(String::from("No URL given")),
                Some(url) => Error::HttpError(format!{"Problem making request to: {}", url}),
            }
        } else if err.is_serialization() {
            Error::SerializeError(String::from("Something went wrong with the serialization."))
        } else if err.is_redirect() {
            Error::RequestError(String::from(
                "Server redirecting too many times or making a loop",
            ))
        } else if err.is_client_error() {
            let status = err.status();

            if let Some(s) = status {
                Error::RequestError(format!{"Response had a {} status code", s})
            } else {
                Error::RequestError(String::from("Response had a 4xx status code"))
            }
        } else if err.is_server_error() {
            let status = err.status();

            if let Some(s) = status {
                Error::RequestError(format!{"Response had a {} status code", s})
            } else {
                Error::RequestError(String::from("Response had a 5xx status code"))
            }
        } else {
            Error::RequestError(String::from(
                "Something went wrong while trying to make a request",
            ))
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JsonError(err)
    }
}
