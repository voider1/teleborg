use super::Method;
use crate::types::File;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to upload a .png file with a sticker for later use in createNewStickerSet and addStickerToSet methods (can be used multiple times). Returns the uploaded File on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct UploadStickerFile {
    /// User identifier of sticker file owner
    pub user_id: i64,
    #[builder(default)]
    /// png_sticker file to send with multipart
    pub png_sticker: String,
}

impl_method!(UploadStickerFile, File, png_sticker);
