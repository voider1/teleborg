use serde::Deserialize;
use crate::types::{PhotoSize};

/// This object represents a video file.
#[derive(Clone, Deserialize, Debug)]
pub struct Video {
    /// Unique identifier for this file
    pub file_id: String,
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

