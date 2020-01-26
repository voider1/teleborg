use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an Update with the field pre_checkout_query. Use this method to respond to such pre-checkout queries. On success, True is returned. Note: The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct AnswerPreCheckoutQuery {
    /// Unique identifier for the query to be answered
    pub pre_checkout_query_id: String,
    /// Specify True if everything is alright (goods are available, etc.) and the bot is
    pub ok: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if ok is False. Error message in human readable form that explains the
    pub error_message: Option<String>,
}

impl_method!(AnswerPreCheckoutQuery, bool);
