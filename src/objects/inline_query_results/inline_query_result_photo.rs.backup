use uuid::Uuid;

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;
use objects::inline_keyboard::InlineKeyboardMarkup;

use super::InlineQueryResultType;

use std::any::Any;

/// Represents a link to a photo. By default,
/// this photo will be sent by the user with optional caption.
/// Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the photo.
#[derive(Serialize)]
pub struct InlineQueryResultPhoto {
    #[serde(rename="type")]
    result_type: String,
    id: String,
    pub photo_url: String,
    pub thumb_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub photo_width: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub photo_height: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

impl InlineQueryResultPhoto {
    pub fn new(photo_url: String,
               thumb_url: String,
               photo_width: Option<i64>,
               photo_height: Option<i64>,
               title: Option<String>,
               description: Option<String>,
               caption: Option<String>,
               reply_markup: Option<InlineKeyboardMarkup>,
               input_message_content: Option<Box<InputMessageContent>>)
               -> Self {
        let result_type = "photo".to_string();
        let id = Uuid::new_v4().simple().to_string();

        InlineQueryResultPhoto {
            result_type: result_type,
            id: id,
            photo_url: photo_url,
            thumb_url: thumb_url,
            photo_width: photo_width,
            photo_height: photo_height,
            title: title,
            description: description,
            caption: caption,
            reply_markup: reply_markup,
            input_message_content: input_message_content,
        }
    }
}

impl InlineQueryResult for InlineQueryResultPhoto {
    fn as_any(&self) -> &Any {
        self
    }

    fn get_type(&self) -> InlineQueryResultType {
        InlineQueryResultType::Photo
    }
}
