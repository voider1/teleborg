use serde_json;

use error::Result;
use message::Message;

#[derive(Deserialize, Debug)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub inline_query: Option<String>, // TODO: Change this into an InlineQuery struct
    pub chosen_inline_result: Option<String>, // TODO: Change this into a ChosenInlineResult struct
    pub callback_query: Option<String>, // TODO: Change into a CallbackQuery struct
}

impl Update {
    pub fn new(update: &serde_json::Value) -> Result<Update> {
        let update: Update = serde_json::from_value(update.clone())?;
        Ok(update)
    }
}
