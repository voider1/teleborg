use uuid::Uuid;

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;
use objects::InlineKeyboardMarkup;
use marker::ReplyMarkup;

use super::InlineQueryResultType;

use std::any::Any;

/// Represents a link to an article or web page.
#[derive(Serialize)]
pub struct InlineQueryResultArticle {
    #[serde(rename="type")]
    result_type: String,
    id: String,
    pub title: String,
    pub input_message_content: Box<InputMessageContent>,
    pub reply_markup: Option<Box<ReplyMarkup>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub hide_url: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub thumb_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub thumb_width: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub thumb_height: Option<i64>,
}

impl InlineQueryResultArticle {
    pub fn new(title: String,
               input_message_content: Box<InputMessageContent>,
               reply_markup: Option<Box<ReplyMarkup>>,
               url: Option<String>,
               hide_url: Option<bool>,
               description: Option<String>,
               thumb_url: Option<String>,
               thumb_width: Option<i64>,
               thumb_height: Option<i64>)
               -> Self {
        let result_type = "article".to_string();
        let id = Uuid::new_v4().simple().to_string();

        InlineQueryResultArticle {
            result_type: result_type,
            id: id,
            title: title,
            input_message_content: input_message_content,
            reply_markup: reply_markup,
            url: url,
            hide_url: hide_url,
            description: description,
            thumb_url: thumb_url,
            thumb_width: thumb_width,
            thumb_height: thumb_height,
        }
    }
}

impl InlineQueryResult for InlineQueryResultArticle {
    fn as_any(&self) -> &Any {
        self
    }

    fn get_type(&self) -> InlineQueryResultType {
        InlineQueryResultType::Article
    }
}
