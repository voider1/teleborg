use uuid::Uuid;

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;
use objects::inline_keyboard::InlineKeyboardMarkup;

use super::InlineQueryResultType;

use std::any::Any;

/// Represents a contact with a phone number.
/// By default, this contact will be sent by the user.
/// Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the contact.
#[derive(Serialize)]
pub struct InlineQueryResultContact {
    #[serde(rename="type")]
    result_type: String,
    id: String,
    phone_number: String,
    first_name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    last_name: Option<String>,
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

impl InlineQueryResultContact {
    pub fn new(phone_number: String,
               first_name: String,
               last_name: Option<String>,
               reply_markup: Option<InlineKeyboardMarkup>,
               input_message_content: Option<Box<InputMessageContent>>,
               thumb_url: Option<String>,
               thumb_width: Option<i64>,
               thumb_height: Option<i64>)
               -> Self {
        let result_type = "contact".to_string();
        let id = format!("{}", Uuid::new_v4());

        InlineQueryResultContact {
            result_type: result_type,
            id: id,
            phone_number: phone_number,
            first_name: first_name,
            last_name: last_name,
            reply_markup: reply_markup,
            input_message_content: input_message_content,
            thumb_url: thumb_url,
            thumb_width: thumb_width,
            thumb_height: thumb_height,
        }
    }
}

impl InlineQueryResult for InlineQueryResultContact {
    fn get_type(&self) -> InlineQueryResultType {
        InlineQueryResultType::Contact
    }

    fn as_any(&self) -> &Any {
        self
    }
}
