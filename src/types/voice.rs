use serde::Deserialize;

/// This object represents a voice note.
#[derive(Clone, Deserialize, Debug)]
pub struct Voice {
    /// Unique identifier for this file
    pub file_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: i64,
    /// Optional. MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// Optional. File size
    pub file_size: Option<i64>,
}

