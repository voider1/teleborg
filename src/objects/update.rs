use objects::InlineQuery;
use objects::ChosenInlineResult;
use objects::CallBackQuery;
use super::Message;

/// Represents an update returned by the Telegram API.
#[derive(Clone, Deserialize, Debug)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub inline_query: Option<InlineQuery>,
    pub chosen_inline_result: Option<ChosenInlineResult>,
    pub callback_query: Option<CallBackQuery>,
}
