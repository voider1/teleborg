use crate::types::PhotoSize;
/// This code is generated using teleborg-api-validator (https://gitlab.com/b.wisman155/teleborg-api-validater)
use serde::Deserialize;

/// This object represents a video message (available in Telegram apps as of v.4.0).
#[derive(Clone, Deserialize, Debug)]
pub struct VideoNote {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width and height (diameter of the video message) as defined by sender
    pub length: i64,
    /// Duration of the video in seconds as defined by sender
    pub duration: i64,
    /// Optional. Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// Optional. File size
    pub file_size: Option<i64>,
}
