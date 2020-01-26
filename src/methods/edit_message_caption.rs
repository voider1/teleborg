/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use super::Method;
use crate::types::{ParseMode, ReplyMarkup, Message};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to edit captions of messages. On success, if edited message is sent by the bot, the edited Message is returned, otherwise True is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct EditMessageCaption {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Unique identifier for the target
    pub chat_id: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Identifier of the message to
    pub message_id: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if chat_id and message_id are not specified. Identifier of the inline
    pub inline_message_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// New caption of the message
    pub caption: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-
    pub parse_mode: Option<ParseMode>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for an inline keyboard.
    pub reply_markup: Option<ReplyMarkup>,
}

impl_method!(EditMessageCaption, Message);
