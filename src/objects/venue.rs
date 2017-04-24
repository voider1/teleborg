use objects::Location;

/// Represents a venue.
#[derive(Clone, Deserialize, Debug)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
}
