use serde::Deserialize;
use crate::types::{ReplyMarkup, InputMessageContent};

/// Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the location.
#[derive(Clone, Deserialize, Debug)]
pub struct InlineQueryResultLocation {
    /// Type of the result, must be location
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Location latitude in degrees
    pub latitude: f64,
    /// Location longitude in degrees
    pub longitude: f64,
    /// Location title
    pub title: String,
    /// Optional. Period in seconds for which the location can be updated, should be between 60 and 86400.
    pub live_period: Option<i64>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<ReplyMarkup>,
    /// Optional. Content of the message to be sent instead of the location
    pub input_message_content: Option<InputMessageContent>,
    /// Optional. Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// Optional. Thumbnail width
    pub thumb_width: Option<i64>,
    /// Optional. Thumbnail height
    pub thumb_height: Option<i64>,
}

