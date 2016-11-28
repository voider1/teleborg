use json;

use message::Message;

struct Update {
    update_id: i32,
    message: Option<Message>,  // TODO: Change this into a Message struct
    edited_message: Option<Message>,  // TODO: Change this into a Message struct
    inline_query: Option<String>,  // TODO: Change this into an InlineQuery struct
    chosen_inline_result: Option<String>,  // TODO: Change this into a ChosenInlineResult struct
    callback_query: Option<String>,  // TODO: Change into a CallbackQuery struct
}

impl Update {
    pub fn new(rjson: json::JsonValue) -> Update {
        let update_id = rjson["update_id"].as_i32().unwrap();

        Update {
            update_id: update_id,
            message: None,
            edited_message: None,
            inline_query: None,
            chosen_inline_result: None,
            callback_query: None,
        }
    }
}
