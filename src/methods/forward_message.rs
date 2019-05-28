use super::Method;
use crate::types::Message;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to forward messages of any kind. On success, the sent `Message` is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct ForwardMessage {
    /// Unique identifier for the target chat.
    pub chat_id: i64,
    /// Unique identifier for the chat where the original message was sent.
    pub from_chat_id: i64,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users get a notification without sound.
    pub disable_notification: Option<bool>,
    /// Message identifier in the chat specified in *from_chat_id*.
    pub message_id: i32,
}

impl_method!(ForwardMessage, Message, "forwardMessage");
