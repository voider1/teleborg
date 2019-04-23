use super::Method;
use crate::types::{Message, ReplyMarkup};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to send phone contacts. On success, the sent `Message` is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendContact {
    /// Unique identifier for the target chat.
    pub chat_id: i64,
    /// Contact's phone number.
    pub phone_number: String,
    /// Contact's first name.
    pub first_name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Contact's last name.
    pub last_name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional data about the contact in the form of
    /// [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes.
    pub vcard: Option<String>,
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

impl_method!(SendContact, Message, "sendContact");
