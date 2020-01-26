/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use super::Method;
use crate::types::{StickerSet};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to get a sticker set. On success, a StickerSet object is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct GetStickerSet {
    /// Name of the sticker set
    pub name: String,
}

impl_method!(GetStickerSet, StickerSet);
