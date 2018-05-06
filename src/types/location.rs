/// This struct represents a point on the map.
#[derive(Clone, Deserialize, Debug)]
pub struct Location {
    /// Longitude as defined by the sender.
    pub longitude: f64,
    /// Latitude as defined by the sender.
    pub latitude: f64,
}
