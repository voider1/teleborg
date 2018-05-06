/// This struct represents a voice note.
#[derive(Clone, Deserialize, Debug)]
pub struct Voice {
    /// Unique identifier for this file.
    pub file_id: String,
    /// Duration of the audio in seconds as defined by the sender.
    pub duration: i64,
    /// MIME type of the file as defined by the sender.
    pub mime_type: Option<String>,
    /// File size.
    pub file_size: Option<i64>,
}
