/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use super::Method;
use crate::types::{ShippingOption};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// If you sent an invoice requesting a shipping address and the parameter is_flexible was specified, the Bot API will send an Update with a shipping_query field to the bot. Use this method to reply to shipping queries. On success, True is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct AnswerShippingQuery {
    /// Unique identifier for the query to be answered
    pub shipping_query_id: String,
    /// Specify True if delivery to the specified address is possible and False if there
    pub ok: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if ok is True. A JSON-serialized array of available shipping options.
    pub shipping_options: Option<Vec<ShippingOption>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if ok is False. Error message in human readable form that explains why
    pub error_message: Option<String>,
}

impl_method!(AnswerShippingQuery, bool);
