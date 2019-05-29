use super::Method;
use crate::types::{Message, ReplyMarkup};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to stop updating a live location message before live_period expires.
/// On success, if the message was sent by the bot, the sent Message is returned,
/// otherwise True is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct StopMessageLiveLocation {
    /// Required if inline_message_id is not specified.
    ///	Unique identifier for the target chat or username of the target channel
    ///	(in the format @channelusername)
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the message to stop
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    ///	Required if chat_id and message_id are not specified. Identifier of the inline message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<i64>,
    /// Interface options.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl_method!(StopMessageLiveLocation, Message, "stopMessageLiveLocation");
