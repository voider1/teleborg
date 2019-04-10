use super::Method;
use crate::types::{Message, ParseMode, ReplyMarkup};

use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to send text messages. On success, the sent `Message` is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendPhoto {
    /// Unique identifier for the target chat.
    pub chat_id: i64,
    /// Photo to send. Pass a file_id as String to send a photo that exists on the Telegram servers (recommended), 
    /// pass an HTTP URL as a String for Telegram to get a photo from the Internet, 
    /// or upload a new photo using multipart/form-data.
    pub photo: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption text for the photo.
    pub caption: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Select the way your message should be parsed using the `ParseMode` struct.
    pub parse_mode: Option<ParseMode>,
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

