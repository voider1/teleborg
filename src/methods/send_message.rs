use super::Method;
use types::{Message, ParseMode, ReplyMarkup};

/// Use this method to send text messages. On success, the sent `Message` is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendMessage {
    chat_id: i64,
    text: &'static str,
    #[default]
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[default]
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_view: Option<bool>,
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

impl_method!(SendMessage, Message, "sendMessage");
