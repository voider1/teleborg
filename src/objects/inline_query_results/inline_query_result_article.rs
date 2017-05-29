use std::time::Instant;

use rand::{Rng, thread_rng};

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultArticle<I> {
    #[serde(rename="type")]
    pub result_type: String,
    pub id: String,
    pub title: String,
    pub input_message_content: Box<I>,
}

impl <I: InputMessageContent>InlineQueryResultArticle<I> {
    pub fn new(title: String, input_message_content: I) -> InlineQueryResultArticle<I> {
        let now = Instant::now();
        let seconds = now.elapsed().as_secs();
        let mut rng = thread_rng();
        let num = rng.gen_range::<i64>(1000, 9999);
        let input_message_content = Box::new(input_message_content);

        InlineQueryResultArticle {
            result_type: "article".to_string(),
            id: format!("{}{}", num, seconds),
            title: title,
            input_message_content: input_message_content,
        }
    }
}

impl <I: InputMessageContent>InlineQueryResult for InlineQueryResultArticle<I> {}
