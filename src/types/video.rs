use super::PhotoSize;
use serde::Deserialize;

/// This struct represents a video file.
#[derive(Clone, Deserialize, Debug)]
pub struct Video {
    /// Unique identifier for this file.
    pub file_id: String,
    /// Video width as defined by the sender.
    pub width: i64,
    /// Video height as defined by the sender.
    pub height: i64,
    /// Duration of the video in seconds as defined by the sender.
    pub duration: i64,
    /// Video thumbnail.
    pub thumb: Option<PhotoSize>,
    /// MIME type of the file as defined by the sender.
    pub mime_type: Option<String>,
    /// File size.
    pub file_size: Option<i64>,
}
