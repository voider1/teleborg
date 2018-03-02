use std::fmt;
use std::fmt::Display;
use std::result;

use failure::{Backtrace, Context, Fail};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "A networking error occured.")]
    NetworkingError,
    #[fail(display = "A JSON serialization error occured.")]
    JSONSerializationError,
    #[fail(display = "A JSON deserialization error occured, body is not in JSON format or is different from the target type.")]
    JSONDeserializationError,
    #[fail(display = "A JSON parsing error occured, could not find key in response")]
    JSONNotFoundError,
    #[fail(display = "An URL parsing error occured.")]
    URLParsingError,
    #[fail(display = "The Telegram bot API therw an error")]
    TelegramAPIError,
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        *self.inner.get_context()
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { inner: Context::new(kind) }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}
