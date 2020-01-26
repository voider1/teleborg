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
    #[builder(default)]
    /// png_sticker file to send with multipart
    pub file: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Png image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    pub png_sticker: Option<String>,
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for position where the mask should be placed on faces
    pub mask_position: Option<MaskPosition>,
}

impl_method_multipart!(AddStickerToSet, bool, "addStickerToSet", "png_sticker");
