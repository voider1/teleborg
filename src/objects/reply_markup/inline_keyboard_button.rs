/// Represents one button of an `InlineKeyboardMarkup`, you must use exactly one of the optional fields.
#[derive(Serialize, Debug)]
pub struct InlineKeyboardButton {
    text: String,
    url: Option<String>,
    callback_data: Option<String>,
    switch_inline_query: Option<String>,
    switch_inline_query_current_chat: Option<String>,
}

impl InlineKeyboardButton {
    /// Constructs a new `InlineKeyboardButton`.
    pub fn new(
        text: String,
        url: Option<String>,
        callback_data: Option<String>,
        switch_inline_query: Option<String>,
        switch_inline_query_current_chat: Option<String>,
    ) -> InlineKeyboardButton {
        InlineKeyboardButton {
            text: text,
            url: Some(url.unwrap_or_else(|| "".to_string())),
            callback_data: Some(callback_data.unwrap_or_else(|| "".to_string())),
            switch_inline_query: Some(switch_inline_query.unwrap_or_else(|| "".to_string())),
            switch_inline_query_current_chat: Some(
                switch_inline_query_current_chat.unwrap_or_else(|| "".to_string()),
            ),
        }
    }
}
