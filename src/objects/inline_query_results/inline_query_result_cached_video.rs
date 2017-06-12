use uuid::Uuid;

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;
use objects::inline_keyboard::InlineKeyboardMarkup;

use super::InlineQueryResultType;

use std::any::Any;

/// Represents a link to a video stored on the Telegram servers.
/// By default, this video will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the video.
#[derive(Serialize)]
pub struct InlineQueryResultCachedVideo {
    #[serde(rename="type")]
    result_type: String,
    id: String,
    pub video_file_id: String,
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

impl InlineQueryResultCachedVideo {
    pub fn new(video_file_id: String,
               title: Option<String>,
               description: Option<String>,
               caption: Option<String>,
               reply_markup: Option<InlineKeyboardMarkup>,
               input_message_content: Option<Box<InputMessageContent>>)
               -> Self {
        let result_type = "video".to_string();
        let id = format!("{}", Uuid::new_v4());

        InlineQueryResultCachedVideo {
            result_type: result_type,
            id: id,
            video_file_id: video_file_id,
            title: title,
            description: description,
            caption: caption,
            reply_markup: reply_markup,
            input_message_content: input_message_content,
        }
    }
}

impl InlineQueryResult for InlineQueryResultCachedVideo {
    fn as_any(&self) -> &Any {
        self
    }

    fn get_type(&self) -> InlineQueryResultType {
        InlineQueryResultType::CachedVideo
    }
}
