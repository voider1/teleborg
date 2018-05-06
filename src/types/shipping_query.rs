use super::{ShippingAddress, User};

/// This struct contains information about an incoming shipping query.
#[derive(Clone, Deserialize, Debug)]
pub struct ShippingQuery {
    /// Unique identifier.
    pub id: String,
    /// User who sent the query.
    pub from: User,
    /// Bot specified invoice payload.
    pub invoice_payload: String,
    /// User specified shipping address.
    pub shipping_address: ShippingAddress,
}
