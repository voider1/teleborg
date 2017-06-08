use uuid::Uuid;

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;
use objects::inline_keyboard::InlineKeyboardMarkup;

use super::InlineQueryResultType;

use std::any::Any;

/// Represents a link to a page containing an embedded video player or a video file.
/// By default, this video file will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the video.
#[derive(Serialize)]
pub struct InlineQueryResultVideo {
    #[serde(rename="type")]
    result_type: String,
    id: String,
    pub video_url: String,
    pub mime_type: String,
    pub thumb_url: String,
    pub title: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub video_width: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub video_height: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub video_duration: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

impl InlineQueryResultVideo {
    pub fn new(video_url: String,
               mime_type: String,
               thumb_url: String,
               title: String,
               caption: Option<String>,
               video_width: Option<i64>,
               video_height: Option<i64>,
               video_duration: Option<i64>,
               description: Option<String>,
               reply_markup: Option<InlineKeyboardMarkup>,
               input_message_content: Option<Box<InputMessageContent>>)
               -> Self {
        let result_type = "video".to_string();
        let id = format!("{}", Uuid::new_v4());

        InlineQueryResultVideo {
            result_type: result_type,
            id: id,
            video_url: video_url,
            mime_type: mime_type,
            thumb_url: thumb_url,
            title: title,
            caption: caption,
            video_width: video_width,
            video_height: video_height,
            video_duration: video_duration,
            description: description,
            reply_markup: reply_markup,
            input_message_content: input_message_content,
        }
    }
}

impl InlineQueryResult for InlineQueryResultVideo {
    fn get_type(&self) -> InlineQueryResultType {
        InlineQueryResultType::Video
    }

    fn as_any(&self) -> &Any {
        self
    }
}
