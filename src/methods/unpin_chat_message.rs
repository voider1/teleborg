use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to unpin a message in a group, a supergroup, or a channel. The bot must be an
/// administrator in the chat for this to work and must have the ‘can_pin_messages’ admin right in
/// the supergroup or ‘can_edit_messages’ admin right in the channel. Returns True on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct UnpinChatMessage {
    /// Unique identifier for the target chat.
    pub chat_id: i64,
}

impl_method!(UnpinChatMessage, bool, "unpinChatMessage");