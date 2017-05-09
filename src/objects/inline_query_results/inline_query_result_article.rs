extern crate rand;

use std::time::Instant;

use self::rand::Rng;

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

impl <T:InputMessageContent> InlineQueryResultArticle <T> {
    pub fn new(title: String, input_message_content: T) -> Self {
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

impl <T: InputMessageContent> InlineQueryResult for InlineQueryResultArticle<T> {}
