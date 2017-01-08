use serde_json;

use error::Result;
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

impl Update {
    pub fn new(update: &serde_json::Value) -> Result<Self> {
        let update: Update = serde_json::from_value(update.clone())?;
        Ok(update)
    }
}
