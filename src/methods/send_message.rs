use super::Method;
use crate::types::{Message, ParseMode, ReplyMarkup};

use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to send text messages. On success, the sent `Message` is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendMessage {
    /// Unique identifier for the target chat.
    pub chat_id: i64,
    /// Text of the message to be sent.
    pub text: &'static str,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Select the way your message should be parsed using the `ParseMode` struct.
    pub parse_mode: Option<ParseMode>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disable link previews for links in messages
    pub disable_web_page_view: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users get a notification without sound.
    pub disable_notification: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message.
    pub reply_to_message_id: Option<i32>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Interface options.
    pub reply_markup: Option<ReplyMarkup>,
}

impl_method!(SendMessage, Message, "sendMessage");
