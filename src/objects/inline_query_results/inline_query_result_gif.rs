use uuid::Uuid;

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;
use objects::inline_keyboard::InlineKeyboardMarkup;

use super::InlineQueryResultType;

use std::any::Any;

#[derive(Serialize)]
pub struct InlineQueryResultGif {
    #[serde(rename="type")]
    result_type: String,
    id: String,
    pub gif_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub gif_width: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub gif_height: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub gif_duration: Option<i64>,
    pub thumb_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

/// Represents a link to an animated GIF file. By default,
/// this animated GIF file will be sent by the user with optional caption.
/// Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the animation.
impl InlineQueryResultGif {
    pub fn new(gif_url: String,
               gif_width: Option<i64>,
               gif_height: Option<i64>,
               gif_duration: Option<i64>,
               thumb_url: String,
               title: Option<String>,
               caption: Option<String>,
               reply_markup: Option<InlineKeyboardMarkup>,
               input_message_content: Option<Box<InputMessageContent>>) -> Self {
        let result_type = "gif".to_string();
        let id = format!("{}", Uuid::new_v4());

        InlineQueryResultGif {
            result_type: result_type,
            id: id,
            gif_url: gif_url,
            gif_width: gif_width,
            gif_height: gif_height,
            gif_duration: gif_duration,
            thumb_url: thumb_url,
            title: title,
            caption: caption,
            reply_markup: reply_markup,
            input_message_content: input_message_content,
        }
    }
}

impl InlineQueryResult for InlineQueryResultGif {
    fn get_type(&self) -> InlineQueryResultType {
        InlineQueryResultType::Gif
    }

    fn as_any(&self) -> &Any {
        self
    }
}
