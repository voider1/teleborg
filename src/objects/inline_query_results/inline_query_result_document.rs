use uuid::Uuid;

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;
use objects::inline_keyboard::InlineKeyboardMarkup;

use super::InlineQueryResultType;

use std::any::Any;

/// Represents a link to a file.
/// By default, this file will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the file.
/// Currently, only .PDF and .ZIP files can be sent using this method.
#[derive(Serialize)]
pub struct InlineQueryResultDocument {
    #[serde(rename="type")]
    result_type: String,
    id: String,
    pub title: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub caption: Option<String>,
    pub document_url: String,
    pub mime_type: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_message_content: Option<Box<InputMessageContent>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub thumb_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub thumb_width: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub thumb_height: Option<i64>,
}

impl InlineQueryResultDocument {
    pub fn new(title: String,
               caption: Option<String>,
               document_url: String,
               mime_type: String,
               description: Option<String>,
               reply_markup: Option<InlineKeyboardMarkup>,
               input_message_content: Option<Box<InputMessageContent>>,
               thumb_url: Option<String>,
               thumb_width: Option<i64>,
               thumb_height: Option<i64>)
               -> Self {
        let result_type = "document".to_string();
        let id = format!("{}", Uuid::new_v4());

        InlineQueryResultDocument {
            result_type: result_type,
            id: id,
            title: title,
            caption: caption,
            document_url: document_url,
            mime_type: mime_type,
            description: description,
            reply_markup: reply_markup,
            input_message_content: input_message_content,
            thumb_url: thumb_url,
            thumb_width: thumb_width,
            thumb_height: thumb_height,
        }
    }
}

impl InlineQueryResult for InlineQueryResultDocument {
    fn get_type(&self) -> InlineQueryResultType {
        InlineQueryResultType::Document
    }

    fn as_any(&self) -> &Any {
        self
    }
}
