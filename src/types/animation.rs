use super::PhotoSize;
use serde::Deserialize;

/// This struct represents an animation file to be displayed in the message containing a `Game`.
#[derive(Clone, Deserialize, Debug)]
pub struct Animation {
    /// Unique file identifier.
    pub file_id: String,
    /// Animation thumbnail as defined by the sender.
    pub thumb: Option<PhotoSize>,
    /// Original animation filename as defined by the sender.
    pub file_name: Option<String>,
    /// MIME type of the file as defined by the sender.
    pub mime_type: Option<String>,
    /// File size.
    pub file_size: Option<i64>,
}
