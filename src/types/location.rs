use serde::Deserialize;

/// This object represents a point on the map.
#[derive(Clone, Deserialize, Debug)]
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: f64,
    /// Latitude as defined by sender
    pub latitude: f64,
}

