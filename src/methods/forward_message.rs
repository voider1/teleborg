/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use super::Method;
use crate::types::{Message};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to forward messages of any kind. On success, the sent Message is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct ForwardMessage {
    /// Unique identifier for the target chat or username of the target channel (in the
    pub chat_id: i64,
    /// Unique identifier for the chat where the original message was sent (or channel
    pub from_chat_id: i64,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Message identifier in the chat specified in from_chat_id
    pub message_id: i64,
}

impl_method!(ForwardMessage, Message);
