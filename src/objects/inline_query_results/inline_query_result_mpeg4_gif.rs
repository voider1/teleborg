use uuid::Uuid;

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;
use objects::InlineKeyboardMarkup;

use super::InlineQueryResultType;

use std::any::Any;

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound).
/// By default, this animated MPEG-4 file will be sent by the user with optional caption.
/// Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the animation.
#[derive(Serialize)]
pub struct InlineQueryResultMpeg4Gif {
    #[serde(rename="type")]
    result_type: String,
    id: String,
    pub mpeg4_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub mpeg4_width: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub mpeg4_height: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub mpeg4_duration: Option<i64>,
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

impl InlineQueryResultMpeg4Gif {
    pub fn new(mpeg4_url: String,
               mpeg4_width: Option<i64>,
               mpeg4_height: Option<i64>,
               mpeg4_duration: Option<i64>,
               thumb_url: String,
               title: Option<String>,
               caption: Option<String>,
               reply_markup: Option<InlineKeyboardMarkup>,
               input_message_content: Option<Box<InputMessageContent>>)
               -> Self {
        let result_type = "mpeg4_gif".to_string();
        let id = Uuid::new_v4().simple().to_string();

        InlineQueryResultMpeg4Gif {
            result_type: result_type,
            id: id,
            mpeg4_url: mpeg4_url,
            mpeg4_width: mpeg4_width,
            mpeg4_height: mpeg4_height,
            mpeg4_duration: mpeg4_duration,
            thumb_url: thumb_url,
            title: title,
            caption: caption,
            reply_markup: reply_markup,
            input_message_content: input_message_content,
        }
    }
}

impl InlineQueryResult for InlineQueryResultMpeg4Gif {
    fn get_type(&self) -> InlineQueryResultType {
        InlineQueryResultType::Mpeg4Gif
    }

    fn as_any(&self) -> &Any {
        self
    }
}
