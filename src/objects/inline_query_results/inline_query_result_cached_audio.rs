use uuid::Uuid;

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;
use objects::inline_keyboard::InlineKeyboardMarkup;

use super::InlineQueryResultType;

use std::any::Any;

/// Represents a link to an animated GIF file stored on the Telegram servers.
/// By default, this animated GIF file will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message
/// with specified content instead of the animation.
#[derive(Serialize)]
pub struct InlineQueryResultCachedAudio {
    #[serde(rename="type")]
    result_type: String,
    id: String,
    pub audio_file_id: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

impl InlineQueryResultCachedAudio {
    pub fn new(audio_file_id: String,
               caption: Option<String>,
               reply_markup: Option<InlineKeyboardMarkup>,
               input_message_content: Option<Box<InputMessageContent>>)
               -> Self {
        let result_type = "audio".to_string();
        let id = format!("{}", Uuid::new_v4());

        InlineQueryResultCachedAudio {
            result_type: result_type,
            id: id,
            audio_file_id: audio_file_id,
            caption: caption,
            reply_markup: reply_markup,
            input_message_content: input_message_content,
        }
    }
}

impl InlineQueryResult for InlineQueryResultCachedAudio {
    fn as_any(&self) -> &Any {
        self
    }

    fn get_type(&self) -> InlineQueryResultType {
        InlineQueryResultType::CachedAudio
    }
}
