/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use super::Method;
use crate::types::{InlineQueryResult};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to send answers to an inline query. On success, True is returned.No more than 50 results per query are allowed.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct AnswerInlineQuery {
    /// Unique identifier for the answered query
    pub inline_query_id: String,
    /// A JSON-serialized array of results for the inline query
    pub results: Vec<InlineQueryResult>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The maximum amount of time in seconds that the result of the inline query may be
    pub cache_time: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True, if results may be cached on the server side only for the user that
    pub is_personal: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass the offset that a client should send in the next query with the same text
    pub next_offset: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If passed, clients will display a button with specified text that switches the
    pub switch_pm_text: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Deep-linking parameter for the /start message sent to the bot when user presses
    pub switch_pm_parameter: Option<String>,
}

impl_method!(AnswerInlineQuery, bool);
