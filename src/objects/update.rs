use objects::message::Message;

#[derive(Clone, Deserialize, Debug)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub inline_query: Option<String>,
    pub chosen_inline_result: Option<String>,
    pub callback_query: Option<String>,
}