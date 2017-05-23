extern crate rand;

use std::time::Instant;

use self::rand::Rng;

use objects::input_message_content::InputMessageContent;
use objects::inline_query_results::InlineQueryResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultArticle<I: InputMessageContent> {
    #[serde(rename="type")]
    pub result_type: String,
    pub id: String,
    pub title: String,
    pub input_message_content: Box<I>,
}

impl <I>InlineQueryResultArticle<I> where I: InputMessageContent  {
    pub fn new<T: InputMessageContent>(title: String, input_message_content: Box<T>) -> InlineQueryResultArticle<T> {
        let now = Instant::now();
        let seconds = now.elapsed().as_secs();

        let mut rng = rand::thread_rng();
        let num = rng.gen_range::<i64>(1000, 9999);

        InlineQueryResultArticle {
            result_type: "article".to_owned(),
            id: format!("{}{}", num, seconds),
            title: title,
            input_message_content: input_message_content,
        }
    }
}

impl <I>InlineQueryResult for InlineQueryResultArticle<I> where I: InputMessageContent {}
