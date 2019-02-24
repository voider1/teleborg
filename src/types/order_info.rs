use super::ShippingAddress;

use serde::Deserialize;

/// This struct represents information about an order.
#[derive(Clone, Deserialize, Debug)]
pub struct OrderInfo {
    /// User name.
    pub name: Option<String>,
    /// User's phone number.
    pub phone_number: Option<String>,
    /// User's email.
    pub email: Option<String>,
    /// User's shipping address.
    pub shipping_address: ShippingAddress,
}
