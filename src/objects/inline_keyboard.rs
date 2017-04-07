use std::option::Option;

#[derive(Serialize, Debug)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Serialize, Debug)]
pub struct InlineKeyboardButton {
    text: String,
    url: Option<String>,
    callback_data: Option<String>,
    switch_inline_query: Option<String>,
    switch_inline_query_current_chat: Option<String>,
}

impl InlineKeyboardMarkup {
    pub fn new(inline_keyboard: Vec<Vec<InlineKeyboardButton>>) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup { inline_keyboard: inline_keyboard }
    }
}

impl InlineKeyboardButton {
    pub fn new(text: String,
               url: Option<String>,
               callback_data: Option<String>,
               switch_inline_query: Option<String>,
               switch_inline_query_current_chat: Option<String>)
               -> InlineKeyboardButton {
        InlineKeyboardButton {
            text: text,
            url: Some(url.unwrap_or("".to_string())),
            callback_data: Some(callback_data.unwrap_or("".to_string())),
            switch_inline_query: Some(switch_inline_query.unwrap_or("".to_string())),
            switch_inline_query_current_chat: Some(switch_inline_query_current_chat
                                                       .unwrap_or("".to_string())),
        }
    }
}
