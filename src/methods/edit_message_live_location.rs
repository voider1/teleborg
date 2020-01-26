use super::Method;
use crate::types::{Message, ReplyMarkup};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// This code is generated using teleborg-api-validator
/// Use this method to edit live location messages. A location can be edited until its live_period expires or editing is explicitly disabled by a call to stopMessageLiveLocation. On success, if the edited message was sent by the bot, the edited Message is returned, otherwise True is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct EditMessageLiveLocation {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    pub message_id: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// Latitude of new location
    pub latitude: f64,
    /// Longitude of new location
    pub longitude: f64,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for a new inline keyboard.
    pub reply_markup: Option<ReplyMarkup>,
}

impl_method!(EditMessageLiveLocation, Message, "editMessageLiveLocation");
