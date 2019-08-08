use super::Method;
use crate::types::{Message};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to send a group of photos or videos as an album. On success, an array of the sent Messages is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendMediaGroup {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// A JSON-serialized array describing photos and videos to be sent, must include 2–10 items
    pub media: Vec<1>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the messages silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the messages are a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
}

impl_method!(SendMediaGroup, Vec<Message>);
