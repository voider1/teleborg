use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to send answers to callback queries sent from inline keyboards. The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, True is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct AnswerCallbackQuery {
    /// Unique identifier for the query to be answered
    pub callback_query_id: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Text of the notification. If not specified, nothing will be shown to the user,
    pub text: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If true, an alert will be shown by the client instead of a notification at the
    pub show_alert: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// URL that will be opened by the user's client. If you have created a Game and
    pub url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The maximum amount of time in seconds that the result of the callback query may
    pub cache_time: Option<i64>,
}

impl_method!(AnswerCallbackQuery, bool);
