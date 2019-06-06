use serde::Deserialize;
use crate::types::{PhotoSize};

/// This object represents a video message (available in Telegram apps as of v.4.0).
#[derive(Clone, Deserialize, Debug)]
pub struct VideoNote {
    /// Unique identifier for this file
    pub file_id: String,
    /// Video width and height (diameter of the video message) as defined by sender
    pub length: i64,
    /// Duration of the video in seconds as defined by sender
    pub duration: i64,
    /// Optional. Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// Optional. File size
    pub file_size: Option<i64>,
}

