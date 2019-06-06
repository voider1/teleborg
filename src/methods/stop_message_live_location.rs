use super::Method;
use crate::types::{ReplyMarkup, Message};use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to stop updating a live location message before live_period expires. On success, if the message was sent by the bot, the sent Message is returned, otherwise True is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct StopMessageLiveLocation {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Identifier of the message with live location to stop
    pub message_id: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for a new inline keyboard.
    pub reply_markup: Option<ReplyMarkup>,
}

impl_method!(StopMessageLiveLocation, Message, "stopMessageLiveLocation");