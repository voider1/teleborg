use crate::types::PhotoSize;
use serde::Deserialize;

/// This object represents a video file.
#[derive(Clone, Deserialize, Debug)]
pub struct Video {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width as defined by sender
    pub width: i64,
    /// Video height as defined by sender
    pub height: i64,
    /// Duration of the video in seconds as defined by sender
    pub duration: i64,
    /// Optional. Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// Optional. Mime type of a file as defined by sender
    pub mime_type: Option<String>,
    /// Optional. File size
    pub file_size: Option<i64>,
}
