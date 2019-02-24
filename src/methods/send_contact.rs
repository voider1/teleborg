use super::Method;
use crate::types::{Message, ReplyMarkup};

use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to send phone contacts. On success, the sent `Message` is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendContact {
    chat_id: i32,
    phone_number: &'static str,
    first_name: &'static str,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<&'static str>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i32>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl_method!(SendContact, Message, "sendContact");
