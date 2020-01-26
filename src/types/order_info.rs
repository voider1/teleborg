use crate::types::ShippingAddress;
/// This code is generated using teleborg-api-validator (https://gitlab.com/b.wisman155/teleborg-api-validater)
use serde::Deserialize;

/// This object represents information about an order.
#[derive(Clone, Deserialize, Debug)]
pub struct OrderInfo {
    /// Optional. User name
    pub name: Option<String>,
    /// Optional. User's phone number
    pub phone_number: Option<String>,
    /// Optional. User email
    pub email: Option<String>,
    /// Optional. User shipping address
    pub shipping_address: Option<ShippingAddress>,
}
