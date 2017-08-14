use uuid::Uuid;

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;
use objects::inline_keyboard::InlineKeyboardMarkup;

use super::InlineQueryResultType;

use std::any::Any;

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on
/// the Telegram servers.
/// By default, this animated MPEG-4 file will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the animation.
#[derive(Serialize)]
pub struct InlineQueryResultCachedMpeg4Gif {
    #[serde(rename="type")]
    result_type: String,
    id: String,
    pub mpeg4_file_id: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

impl InlineQueryResultCachedMpeg4Gif {
    pub fn new(mpeg4_file_id: String,
               title: Option<String>,
               caption: Option<String>,
               reply_markup: Option<InlineKeyboardMarkup>,
               input_message_content: Option<Box<InputMessageContent>>)
               -> Self {
        let result_type = "mpeg4_gif".to_string();
        let id = Uuid::new_v4().simple().to_string();

        InlineQueryResultCachedMpeg4Gif {
            result_type: result_type,
            id: id,
            mpeg4_file_id: mpeg4_file_id,
            title: title,
            caption: caption,
            reply_markup: reply_markup,
            input_message_content: input_message_content,
        }
    }
}

impl InlineQueryResult for InlineQueryResultCachedMpeg4Gif {
    fn as_any(&self) -> &Any {
        self
    }

    fn get_type(&self) -> InlineQueryResultType {
        InlineQueryResultType::CachedMpeg4Gif
    }
}
