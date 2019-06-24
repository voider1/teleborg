use serde::Deserialize;

/// This object represents a file ready to be downloaded. The file can be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile.
#[derive(Clone, Deserialize, Debug)]
pub struct File {
    /// Unique identifier for this file
    pub file_id: String,
    /// Optional. File size, if known
    pub file_size: Option<i64>,
    /// Optional. File path. Use https://api.telegram.org/file/bot<token>/<file_path> to get the file.
    pub file_path: Option<String>,
}
