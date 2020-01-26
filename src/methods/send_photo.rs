/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use super::Method;
use crate::types::{ParseMode, ReplyMarkup, Message};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to send photos. On success, the sent Message is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the
    pub chat_id: i64,
    /// Photo to send. Pass a file_id as String to send a photo that exists on the
    pub photo: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Photo caption (may also be used when resending photos by file_id), 0-1024
    pub caption: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-
    pub parse_mode: Option<ParseMode>,
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

impl_method!(SendPhoto, Message, photo);
