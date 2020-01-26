use crate::types::LabeledPrice;
use serde::{Deserialize, Serialize};

/// This code is generated using teleborg-api-validator
/// This object represents one shipping option.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ShippingOption {
    /// Shipping option identifier
    pub id: String,
    /// Option title
    pub title: String,
    /// List of price portions
    pub prices: Vec<LabeledPrice>,
}
