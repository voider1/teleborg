use crate::types::{MaskPosition, PhotoSize};
use serde::Deserialize;

/// This object represents a sticker.
#[derive(Clone, Deserialize, Debug)]
pub struct Sticker {
    /// Unique identifier for this file
    pub file_id: String,
    /// Sticker width
    pub width: i64,
    /// Sticker height
    pub height: i64,
    /// Optional. Sticker thumbnail in the .webp or .jpg format
    pub thumb: Option<PhotoSize>,
    /// Optional. Emoji associated with the sticker
    pub emoji: Option<String>,
    /// Optional. Name of the sticker set to which the sticker belongs
    pub set_name: Option<String>,
    /// Optional. For mask stickers, the position where the mask should be placed
    pub mask_position: Option<MaskPosition>,
    /// Optional. File size
    pub file_size: Option<i64>,
}
