use serde::Deserialize;

/// Represents the content of a venue message to be sent as the result of an inline query.
#[derive(Clone, Deserialize, Debug)]
pub struct InputVenueMessageContent {
    /// Latitude of the venue in degrees
    pub latitude: f64,
    /// Longitude of the venue in degrees
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Optional. Foursquare identifier of the venue, if known
    pub foursquare_id: Option<String>,
    /// Optional. Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
}

