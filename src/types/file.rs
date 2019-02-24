use serde::Deserialize;

/// This struct represents a Telegram [File](https://core.telegram.org/bots/api#file).
#[derive(Clone, Deserialize, Debug)]
pub struct File {
    /// Unique identifier for this file.
    pub file_id: String,
    /// The size of the file, if it's known.
    pub file_size: Option<i64>,
    /// The path of the file.
    pub file_path: Option<String>,
}
