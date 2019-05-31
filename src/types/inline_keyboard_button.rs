use serde::Serialize;
use typed_builder::TypedBuilder;

/// Represents one button of an `InlineKeyboardMarkup`, you must use exactly one of the optional fields.
#[derive(Debug, Default, Clone, TypedBuilder, Serialize)]
pub struct InlineKeyboardButton {
    text: String,
    url: Option<String>,
    callback_data: Option<String>,
    switch_inline_query: Option<String>,
    switch_inline_query_current_chat: Option<String>,
}
