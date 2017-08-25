use uuid::Uuid;

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;
use objects::InlineKeyboardMarkup;

use super::InlineQueryResultType;

use std::any::Any;

/// Represents a link to a voice recording in an .ogg container encoded with OPUS.
/// By default, this voice recording will be sent by the user.
/// Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the the voice message.
#[derive(Serialize)]
pub struct InlineQueryResultVoice {
    #[serde(rename="type")]
    result_type: String,
    id: String,
    pub voice_url: String,
    pub title: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub voice_duration: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

impl InlineQueryResultVoice {
    pub fn new(voice_url: String,
               title: String,
               caption: Option<String>,
               voice_duration: Option<i64>,
               reply_markup: Option<InlineKeyboardMarkup>,
               input_message_content: Option<Box<InputMessageContent>>)
               -> Self {
        let result_type = "voice".to_string();
        let id = Uuid::new_v4().simple().to_string();

        InlineQueryResultVoice {
            result_type: result_type,
            id: id,
            voice_url: voice_url,
            title: title,
            caption: caption,
            voice_duration: voice_duration,
            reply_markup: reply_markup,
            input_message_content: input_message_content,
        }
    }
}

impl InlineQueryResult for InlineQueryResultVoice {
    fn get_type(&self) -> InlineQueryResultType {
        InlineQueryResultType::Voice
    }

    fn as_any(&self) -> &Any {
        self
    }
}
