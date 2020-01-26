use super::Method;
use crate::types::{Message, ReplyMarkup};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// This code is generated using teleborg-api-validator
/// Use this method to send information about a venue. On success, the sent Message is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendVenue {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Latitude of the venue
    pub latitude: f64,
    /// Longitude of the venue
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
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

impl_method!(SendVenue, Message, "sendVenue");
