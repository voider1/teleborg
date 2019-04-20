use super::Method;
use crate::types::{Message, ReplyMarkup};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to point on the map. On success, the sent `Message` is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct EditMessageLiveLocation {
    /// Required if inline_message_id is not specified.
    ///	Unique identifier for the target chat or username of the target channel
    ///	(in the format @channelusername)
    #[builder(default)]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    #[builder(default)]
    pub message_id: Option<i32>,
    ///	Required if chat_id and message_id are not specified. Identifier of the inline message
    #[builder(default)]
    pub inline_message_id: Option<i64>,
    /// Latitude of the location.
    #[builder(default)]
    pub latitude: f64,
    /// Longitude of the location.
    #[builder(default)]
    pub longitude: f64,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Interface options.
    pub reply_markup: Option<ReplyMarkup>,
}

impl_method!(EditMessageLiveLocation, Message, "editMessageLiveLocation");
