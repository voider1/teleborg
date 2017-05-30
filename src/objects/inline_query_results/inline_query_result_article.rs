use uuid::Uuid;

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;
use objects::inline_keyboard::InlineKeyboardMarkup;

#[derive(Serialize, Debug)]
pub struct InlineQueryResultArticle<I> {
    #[serde(rename="type")]
    pub result_type: String,
    pub id: String,
    pub title: String,
    pub input_message_content: Box<I>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub url: Option<String>,
    pub hide_url: Option<bool>,
    pub description: Option<String>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

impl <I: InputMessageContent>InlineQueryResultArticle<I> {
    pub fn new(title: String,
               input_message_content: I,
               reply_markup: Option<InlineKeyboardMarkup>,
               url: Option<String>,
               hide_url: Option<bool>,
               description: Option<String>,
               thumb_url: Option<String>,
               thumb_width: Option<i64>,
               thumb_height: Option<i64>) -> InlineQueryResultArticle<I> {
        let result_type = "article".to_string();
        let uuid = format!("{}", Uuid::new_v4());
        let input_message_content = Box::new(input_message_content);

        InlineQueryResultArticle {
            result_type: result_type,
            id: uuid,
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

impl <I: InputMessageContent>InlineQueryResult for InlineQueryResultArticle<I> {}
