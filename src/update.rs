use message::Message;

pub struct Update {
    update_id: i32,
    message: Option<Message>,
    edited_message: Option<Message>,
    inline_query: Option<String>,  // TODO: Change this into an InlineQuery struct
    chosen_inline_result: Option<String>,  // TODO: Change this into a ChosenInlineResult struct
    callback_query: Option<String>,  // TODO: Change into a CallbackQuery struct
}

impl Update {
    pub fn new() -> Update {
        Update {
            update_id: 0,
            message: None,
            edited_message: None,
            inline_query: None,
            chosen_inline_result: None,
            callback_query: None,
        }
    }
}
