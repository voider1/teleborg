use serde::Deserialize;
use crate::types::{PhotoSize};

/// This object represents a general file (as opposed to photos, voice messages and audio files).
#[derive(Clone, Deserialize, Debug)]
pub struct Document {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Optional. Document thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Optional. Original filename as defined by sender
    pub file_name: Option<String>,
    /// Optional. MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// Optional. File size
    pub file_size: Option<i64>,
}

