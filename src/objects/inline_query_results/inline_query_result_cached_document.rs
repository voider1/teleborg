use uuid::Uuid;

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;
use objects::inline_keyboard::InlineKeyboardMarkup;

use super::InlineQueryResultType;

use std::any::Any;

/// Represents a link to a document stored on the Telegram servers.
/// By default, this document will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the document.
#[derive(Serialize)]
pub struct InlineQueryResultCachedDocument {
    #[serde(rename="type")]
    result_type: String,
    id: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    pub document_file_id: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
}

impl InlineQueryResultCachedDocument {
    pub fn new(title: Option<String>,
               document_file_id: String,
               description: Option<String>,
               caption: Option<String>,
               reply_markup: Option<InlineKeyboardMarkup>,
               input_message_content: Option<Box<InputMessageContent>>)
               -> Self {
        let result_type = "document".to_string();
        let id = format!("{}", Uuid::new_v4());

        InlineQueryResultCachedDocument {
            result_type: result_type,
            id: id,
            title: title,
            document_file_id: document_file_id,
            description: description,
            caption: caption,
            reply_markup: reply_markup,
            input_message_content: input_message_content,
        }
    }
}

impl InlineQueryResult for InlineQueryResultCachedDocument {
    fn as_any(&self) -> &Any {
        self
    }

    fn get_type(&self) -> InlineQueryResultType {
        InlineQueryResultType::CachedDocument
    }
}
