use super::Method;
use types::{Message, ReplyMarkup};

#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendContact {
    chat_id: i32,
    phone_number: &'static str,
    first_name: &'static str,
    #[default]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<&'static str>,
    #[default]
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[default]
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i32>,
    #[default]
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl_method!(SendContact, Message, "sendContact");
