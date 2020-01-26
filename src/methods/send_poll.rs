use super::Method;
use crate::types::{ReplyMarkup, Message};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// This code is generated using teleborg-api-validator
/// Use this method to send a native poll. On success, the sent Message is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendPoll {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Poll question, 1-255 characters
    pub question: String,
    /// List of answer options, 2-10 strings 1-100 characters each
    pub options: Vec<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the poll needs to be anonymous, defaults to True
    pub is_anonymous: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Poll type, “quiz” or “regular”, defaults to “regular”
    #[serde(rename = "type")]
    pub kind: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to False
    pub allows_multiple_answers: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 0-based identifier of the correct answer option, required for polls in quiz mode
    pub correct_option_id: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True, if the poll needs to be immediately closed
    pub is_closed: Option<bool>,
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
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

impl_method!(SendPoll, Message, "sendPoll");
