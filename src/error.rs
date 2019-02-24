use failure::Error as FailureError;
use failure::Fail;

/// Result which uses failure::Error by default.
pub type Result<T> = ::std::result::Result<T, FailureError>;

/// This enum represents all possible errors.
#[derive(Fail, Debug)]
pub enum Error {
    /// Thrown when a networking error occurs.
    #[fail(display = "{}", _0)]
    NetworkingError(#[cause] ::reqwest::Error),
    /// Thrown when a serialization error occurs.
    #[fail(display = "A JSON serialization error occured.")]
    JsonSerializationError,
    /// Thrown when a deserialization error occurs.
    #[fail(display = "{}", _0)]
    JsonDeserializationError(#[cause] ::serde_json::Error),
    /// Thrown when the expected JSON response couldn't be found.
    #[fail(display = "A JSON parsing error occured, could not find key in response")]
    JsonNotFoundError,
    /// Thrown when the Telegram API returns an error.
    #[fail(
        display = "The Telegram server threw an error\nError code: {}\nError description: {}",
        _0, _1
    )]
    TelegramApiError(String, i32),
}
