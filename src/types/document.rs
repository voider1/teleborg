use crate::types::PhotoSize;
use serde::Deserialize;

/// This object represents a general file (as opposed to photos, voice messages and audio files).
#[derive(Clone, Deserialize, Debug)]
pub struct Document {
    /// Unique file identifier
    pub file_id: String,
    /// Optional. Document thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Optional. Original filename as defined by sender
    pub file_name: Option<String>,
    /// Optional. MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// Optional. File size
    pub file_size: Option<i64>,
}
