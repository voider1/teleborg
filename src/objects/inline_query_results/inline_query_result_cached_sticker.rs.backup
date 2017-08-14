use uuid::Uuid;

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;
use objects::inline_keyboard::InlineKeyboardMarkup;

use super::InlineQueryResultType;

use std::any::Any;

/// Represents a link to a sticker stored on the Telegram servers.
/// By default, this sticker will be sent by the user.
/// Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the sticker.
#[derive(Serialize)]
pub struct InlineQueryResultCachedSticker {
    #[serde(rename="type")]
    result_type: String,
    id: String,
    pub sticker_file_id: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

impl InlineQueryResultCachedSticker {
    pub fn new(sticker_file_id: String,
               reply_markup: Option<InlineKeyboardMarkup>,
               input_message_content: Option<Box<InputMessageContent>>)
               -> Self {
        let result_type = "sticker".to_string();
        let id = Uuid::new_v4().simple().to_string();

        InlineQueryResultCachedSticker {
            result_type: result_type,
            id: id,
            sticker_file_id: sticker_file_id,
            reply_markup: reply_markup,
            input_message_content: input_message_content,
        }
    }
}

impl InlineQueryResult for InlineQueryResultCachedSticker {
    fn as_any(&self) -> &Any {
        self
    }

    fn get_type(&self) -> InlineQueryResultType {
        InlineQueryResultType::CachedSticker
    }
}
