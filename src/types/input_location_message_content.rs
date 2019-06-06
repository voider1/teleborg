use serde::Deserialize;

/// Represents the content of a location message to be sent as the result of an inline query.
#[derive(Clone, Deserialize, Debug)]
pub struct InputLocationMessageContent {
    /// Latitude of the location in degrees
    pub latitude: f64,
    /// Longitude of the location in degrees
    pub longitude: f64,
    /// Optional. Period in seconds for which the location can be updated, should be between 60 and 86400.
    pub live_period: Option<i64>,
}

