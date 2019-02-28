use serde::Deserialize;

/// This struct represents one size of a photo or a or a file / sticker thumbnail.
#[derive(Clone, Deserialize, Debug)]
pub struct PhotoSize {
    /// Unique identifier for this file.
    pub file_id: String,
    /// Photo width.
    pub width: i64,
    /// Photo height.
    pub height: i64,
    /// File size.
    pub file_size: Option<i64>,
}