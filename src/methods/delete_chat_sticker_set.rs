use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to delete a group sticker set from a supergroup. The bot must be an
/// administrator in the chat for this to work and must have the appropriate admin rights. Use the
/// field can_set_sticker_set optionally returned in getChat requests to check if the bot can use
/// this method. Returns True on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct DeleteChatStickerSet {
    /// Unique identifier for the target chat.
    pub chat_id: i64,
}

impl_method!(DeleteChatStickerSet, bool, "deleteChatStickerSet");
