/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use serde::Deserialize;use crate::types::{PhotoSize};

/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Clone, Deserialize, Debug)]
pub struct Audio {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: i64,
    /// Optional. Performer of the audio as defined by sender or by audio tags
    pub performer: Option<String>,
    /// Optional. Title of the audio as defined by sender or by audio tags
    pub title: Option<String>,
    /// Optional. MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// Optional. File size
    pub file_size: Option<i64>,
    /// Optional. Thumbnail of the album cover to which the music file belongs
    pub thumb: Option<PhotoSize>,
}

