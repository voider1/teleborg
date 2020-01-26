/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use super::Method;
use crate::types::{ReplyMarkup, Message};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to send static .WEBP or animated .TGS stickers. On success, the sent Message is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendSticker {
    /// Unique identifier for the target chat or username of the target channel (in the
    pub chat_id: i64,
    /// Sticker to send. Pass a file_id as String to send a file that exists on the
    pub sticker: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional interface options. A JSON-serialized object for an inline keyboard,
    pub reply_markup: Option<ReplyMarkup>,
}

impl_method!(SendSticker, Message, sticker);
