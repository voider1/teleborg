use objects::input_message_content::marker::InputMessageContent;

#[derive(Clone, Deserialize, Debug)]
pub struct InlineQueryResultArticle <I: InputMessageContent> {
    #[serde(rename="type")]
    pub result_type: String,
    pub id: String,
    pub title: String,
    pub input_message_content: I,
}