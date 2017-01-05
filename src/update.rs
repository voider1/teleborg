use serde_json;

use error::{Result, Error};
use message::Message;
use ::ValueExtension;

#[derive(Debug)]
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
        let id = update.as_required_i64("update_id")?;
        let message = update.find("message")
            .map_or(Ok(None), |v| Message::new(v).map(|m| Some(m)))?;
        let edited_message = update.find("edited_message")
            .map_or(Ok(None), |v| Message::new(v).map(|m| Some(m)))?;
        let inline_query = None;
        let chosen_inline_result = None;
        let callback_query = None;

        Ok(Update {
            update_id: id,
            message: message,
            edited_message: edited_message,
            inline_query: inline_query,
            chosen_inline_result: chosen_inline_result,
            callback_query: callback_query,
        })
    }
}
