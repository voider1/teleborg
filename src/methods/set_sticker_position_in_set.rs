use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to move a sticker in a set created by the bot to a specific position . Returns True on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SetStickerPositionInSet {
    /// File identifier of the sticker
    pub sticker: String,
    /// New sticker position in the set, zero-based
    pub position: i64,
}

impl_method!(SetStickerPositionInSet, bool);
