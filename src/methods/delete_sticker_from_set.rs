use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to delete a sticker from a set created by the bot. Returns True on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct DeleteStickerFromSet {
    /// File identifier of the sticker
    pub sticker: String,
}

impl_method!(DeleteStickerFromSet, bool, "deleteStickerFromSet");
