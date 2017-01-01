use serde_json;

use error::Result;
use error::Error;
use message::Message;
use ::ValueExtension;

pub struct Update {
    update_id: i64,
    message: Option<Message>,
    edited_message: Option<Message>,
    inline_query: Option<String>,  // TODO: Change this into an InlineQuery struct
    chosen_inline_result: Option<String>,  // TODO: Change this into a ChosenInlineResult struct
    callback_query: Option<String>,  // TODO: Change into a CallbackQuery struct
}

impl Update {
    pub fn new(update: &serde_json::Value) -> Result<Update> {
        let id = update.as_required_i64("update_id")?;

        Ok(Update {
            update_id: id,
            message: None,
            edited_message: None,
            inline_query: None,
            chosen_inline_result: None,
            callback_query: None,
        })
    }
}
