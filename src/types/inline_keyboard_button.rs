use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Represents one button of an `InlineKeyboardMarkup`, you must use exactly one of the optional fields.
#[derive(Debug, Default, Clone, TypedBuilder, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
    text: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    callback_data: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_inline_query: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_inline_query_current_chat: Option<String>,
}
