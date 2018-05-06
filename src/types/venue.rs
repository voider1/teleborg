use super::Location;

/// This struct represents a venue.
#[derive(Clone, Deserialize, Debug)]
pub struct Venue {
    /// Venue location.
    pub location: Location,
    /// Name of the venue.
    pub title: String,
    /// Address of the venue.
    pub address: String,
    /// Foursquare identifier of the venue.
    pub foursquare_id: Option<String>,
}
