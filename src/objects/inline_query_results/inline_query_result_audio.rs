use uuid::Uuid;

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;
use objects::inline_keyboard::InlineKeyboardMarkup;

use super::InlineQueryResultType;

use std::any::Any;

/// Represents a link to an mp3 audio file. By default, this audio file will be sent by the user.
/// Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the audio.
#[derive(Serialize)]
pub struct InlineQueryResultAudio {
    #[serde(rename="type")]
    result_type: String,
    id: String,
    pub audio_url: String,
    pub title: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub performer: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub audio_duration: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

impl InlineQueryResultAudio {
    pub fn new(audio_url: String,
               title: String,
               caption: Option<String>,
               performer: Option<String>,
               audio_duration: Option<i64>,
               reply_markup: Option<InlineKeyboardMarkup>,
               input_message_content: Option<Box<InputMessageContent>>)
               -> Self {
        let result_type = "audio".to_string();
        let id = format!("{}", Uuid::new_v4());

        InlineQueryResultAudio {
            result_type: result_type,
            id: id,
            audio_url: audio_url,
            title: title,
            caption: caption,
            performer: performer,
            audio_duration: audio_duration,
            reply_markup: reply_markup,
            input_message_content: input_message_content,
        }
    }
}

impl InlineQueryResult for InlineQueryResultAudio {
    fn get_type(&self) -> InlineQueryResultType {
        InlineQueryResultType::Audio
    }

    fn as_any(&self) -> &Any {
        self
    }
}
