use uuid::Uuid;

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;
use objects::inline_keyboard::InlineKeyboardMarkup;

use super::InlineQueryResultType;

use std::any::Any;

/// Represents a link to a voice message stored on the Telegram servers.
/// By default, this voice message will be sent by the user.
/// Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the voice message.
#[derive(Serialize)]
pub struct InlineQueryResultCachedVoice {
    #[serde(rename="type")]
    result_type: String,
    id: String,
    pub voice_file_id: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

impl InlineQueryResultCachedVoice {
    pub fn new(voice_file_id: String,
               title: Option<String>,
               caption: Option<String>,
               reply_markup: Option<InlineKeyboardMarkup>,
               input_message_content: Option<Box<InputMessageContent>>)
               -> Self {
        let result_type = "voice".to_string();
        let id = Uuid::new_v4().simple().to_string();

        InlineQueryResultCachedVoice {
            result_type: result_type,
            id: id,
            voice_file_id: voice_file_id,
            title: title,
            caption: caption,
            reply_markup: reply_markup,
            input_message_content: input_message_content,
        }
    }
}

impl InlineQueryResult for InlineQueryResultCachedVoice {
    fn as_any(&self) -> &Any {
        self
    }

    fn get_type(&self) -> InlineQueryResultType {
        InlineQueryResultType::CachedVoice
    }
}
