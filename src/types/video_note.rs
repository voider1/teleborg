use super::PhotoSize;

use serde::Deserialize;

/// This struct represents a [video note](https://telegram.org/blog/video-messages-and-telescope).
#[derive(Clone, Deserialize, Debug)]
pub struct VideoNote {
    /// Unique identifier for this file.
    pub file_id: String,
    /// Video width and length as defined by the sender.
    pub length: i64,
    /// Duration of the video in seconds as defined by the sender.
    pub duration: i64,
    /// Video thumbnail.
    pub thumb: PhotoSize,
    /// File size.
    pub file_size: i64,
}
