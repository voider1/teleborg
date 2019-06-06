use super::Method;
use crate::types::{ReplyMarkup, Message};use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to send point on the map. On success, the sent Message is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendLocation {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Latitude of the location
    pub latitude: f64,
    /// Longitude of the location
    pub longitude: f64,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Period in seconds for which the location will be updated (see Live Locations, should be between 60 and 86400.
    pub live_period: Option<i64>,
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

impl_method!(SendLocation, Message, "sendLocation");