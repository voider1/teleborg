/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use super::Method;
use crate::types::{MaskPosition};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to create new sticker set owned by a user. The bot will be able to edit the created sticker set. Returns True on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct CreateNewStickerSet {
    /// User identifier of created sticker set owner
    pub user_id: i64,
    /// Short name of sticker set, to be used in t.me/addstickers/ URLs (e.g., animals).
    pub name: String,
    /// Sticker set title, 1-64 characters
    pub title: String,
    /// Png image with the sticker, must be up to 512 kilobytes in size, dimensions must
    pub png_sticker: String,
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True, if a set of mask stickers should be created
    pub contains_masks: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for position where the mask should be placed on faces
    pub mask_position: Option<MaskPosition>,
}

impl_method!(CreateNewStickerSet, bool, png_sticker);
