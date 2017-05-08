pub struct InlineQueryResultArticle <I> {
    #[serde(rename="type")]
    pub result_type: String,
    pub id: String,
    pub title: String,
    pub input_message_content: I,
}