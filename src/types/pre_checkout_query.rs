use super::{OrderInfo, User};
use serde::Deserialize;

/// This object contains information about an incoming pre-checkout query.
#[derive(Clone, Deserialize, Debug)]
pub struct PreCheckoutQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145.
    pub total_amount: i128,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// Identifier of the shipping option chosen by the user
    pub shipping_option_id: Option<String>,
    /// Order info provided by the user
    pub order_info: Option<OrderInfo>,
}
