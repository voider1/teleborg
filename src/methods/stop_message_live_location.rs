use super::Method;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::types::{Message, ReplyMarkup};

/// Use this method to stop updating a live location message before live_period expires.
/// On success, if the message was sent by the bot, the sent Message is returned,
/// otherwise True is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct StopMessageLiveLocation {
    /// Required if inline_message_id is not specified.
    ///	Unique identifier for the target chat or username of the target channel
    ///	(in the format @channelusername)
    #[builder(default)]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the message to stop
    #[builder(default)]
    pub message_id: Option<i32>,
    ///	Required if chat_id and message_id are not specified. Identifier of the inline message
    #[builder(default)]
    pub inline_message_id: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Interface options.
    pub reply_markup: Option<ReplyMarkup>,
}

impl_method!(StopMessageLiveLocation, Message, "stopMessageLiveLocation");
