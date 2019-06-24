use crate::types::PhotoSize;
use serde::Deserialize;

/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(Clone, Deserialize, Debug)]
pub struct Animation {
    /// Unique file identifier
    pub file_id: String,
    /// Video width as defined by sender
    pub width: i64,
    /// Video height as defined by sender
    pub height: i64,
    /// Duration of the video in seconds as defined by sender
    pub duration: i64,
    /// Optional. Animation thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Optional. Original animation filename as defined by sender
    pub file_name: Option<String>,
    /// Optional. MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// Optional. File size
    pub file_size: Option<i64>,
}
