/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use super::Method;
use crate::types::{MaskPosition};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to add a new sticker to a set created by the bot. Returns True on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct AddStickerToSet {
    /// User identifier of sticker set owner
    pub user_id: i64,
    /// Sticker set name
    pub name: String,
    /// Png image with the sticker, must be up to 512 kilobytes in size, dimensions must
    pub png_sticker: String,
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for position where the mask should be placed on faces
    pub mask_position: Option<MaskPosition>,
}

impl_method!(AddStickerToSet, bool, png_sticker);
