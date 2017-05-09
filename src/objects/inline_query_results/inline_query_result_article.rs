use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;

#[derive(Deserialize, Serialize)]
pub struct InlineQueryResultArticle <T: InputMessageContent> {
    #[serde(rename="type")]
    pub result_type: String,
    pub id: String,
    pub title: String,
    pub input_message_content: T,
}

impl InlineQueryResultArticle<T> where T: InputMessageContent  {
    pub fn new(title: String, input_message_content: T) -> Self {
        InlineQueryResultArticle {
            result_type: "article",
            id: "",
            title: title,
            input_message_content: input_message_content,
        }
    }
}

impl <T> InlineQueryResult for InlineQueryResultArticle<T> where T: InputMessageContent {}
