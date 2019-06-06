use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to pin a message in a group, a supergroup, or a channel. The bot must be an administrator in the chat for this to work and must have the ‘can_pin_messages’ admin right in the supergroup or ‘can_edit_messages’ admin right in the channel. Returns True on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct PinChatMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Identifier of a message to pin
    pub message_id: i64,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True, if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels.
    pub disable_notification: Option<bool>,
}

impl_method!(PinChatMessage, bool, "pinChatMessage");