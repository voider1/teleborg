use serde::{Deserialize, Serialize};

/// This struct represents an audio file to be treated as music by the Telegram clients.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Audio {
    /// Unique identifier for this file.
    pub file_id: String,
    /// Duration of the audio in seconds as defined by sender.
    pub duration: i64,
    /// Performer of the audio as defined by sender or by audio tags.
    pub performer: Option<String>,
    /// Title of the audio as defined by sender or by audio tags.
    pub title: Option<String>,
    /// MIME type of the file as defined by sender.
    pub mime_type: Option<String>,
    /// File size.
    pub file_size: Option<i64>,
}
