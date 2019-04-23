use super::Method;
use crate::types::{Message, ParseMode, ReplyMarkup};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Change an existing message in the message history instead of sending a new one with a result of
/// an action.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct EditMessageText {
    /// Required if `inline_message_id` is unspecified.
    /// Unique identifier for the target chat, username or target channel.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if `inline_message_id` is unspecified.
    /// Identifier of the sent message.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    /// Required if chat_id and message_id are not specified.
    /// Identifier of the inline messsage.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// New text of the message.
    pub text: String,
    /// Select the way your message should be parsed using the `ParseMode` struct.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users get a notification without sound.
    pub disable_notification: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Interface options.
    pub reply_markup: Option<ReplyMarkup>,
}

impl_method!(EditMessageText, Message, "editMessageText");
