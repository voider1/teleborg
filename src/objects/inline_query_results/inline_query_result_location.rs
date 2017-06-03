use uuid::Uuid;

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;
use objects::inline_keyboard::InlineKeyboardMarkup;

use super::InlineQueryResultType;

use std::any::Any;

/// Represents a location on a map. By default, the location will be sent by the user.
/// Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the location.
#[derive(Serialize)]
pub struct InlineQueryResultLocation {
    #[serde(rename="type")]
    result_type: String,
    id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub thumb_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub thumb_width: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub thumb_height: Option<i64>,
}

impl InlineQueryResultLocation {
    pub fn new(latitude: f64,
               longitude: f64,
               title: String,
               reply_markup: Option<InlineKeyboardMarkup>,
               input_message_content: Option<Box<InputMessageContent>>,
               thumb_url: Option<String>,
               thumb_width: Option<i64>,
               thumb_height: Option<i64>) -> Self {
        let result_type = "location".to_string();
        let id = format!("{}", Uuid::new_v4());

        InlineQueryResultLocation {
            result_type: result_type,
            id: id,
            latitude: latitude,
            longitude: longitude,
            title: title,
            reply_markup: reply_markup,
            input_message_content: input_message_content,
            thumb_url: thumb_url,
            thumb_width: thumb_width,
            thumb_height: thumb_height,
        }
    }
}

impl InlineQueryResult for InlineQueryResultLocation {
    fn get_type(&self) -> InlineQueryResultType {
        InlineQueryResultType::Location
    }

    fn as_any(&self) -> &Any {
        self
    }
}