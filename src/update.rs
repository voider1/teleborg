use json;

use message;

struct Update {
    update_id: i32,
    message: Option<Message>,  // TODO: Change this into a Message struct
    edited_message: Option<Message>,  // TODO: Change this into a Message struct
    inline_query: Option<String>,  // TODO: Change this into an InlineQuery struct
    chosen_inline_result: Option<String>,  // TODO: Change this into a ChosenInlineResult struct
    callback_query: Option<String>,  // TODO: Change into a CallbackQuery struct
}

impl Update {
    pub fn new(rjson: json::JsonValue) {
        let update_id = rjson["update_id"];
    }
}
