use super::{InputMessageContent, InputMessageType};

use std::any::Any;

#[derive(Serialize, Deserialize)]
pub struct InputVenueMessageContent {
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub foursquare_id: Option<String>,
}

impl InputVenueMessageContent {
    pub fn new(latitude: f64,
               longitude: f64,
               title: String,
               address: String,
               foursquare_id: Option<String>) -> Self {
        InputVenueMessageContent {
            latitude: latitude,
            longitude: longitude,
            title: title,
            address: address,
            foursquare_id: foursquare_id,
        }
    }
}

impl InputMessageContent for InputVenueMessageContent {
    fn as_any(&self) -> &Any {
        self
    }

    fn get_type(&self) -> InputMessageType {
        InputMessageType::Venue
    }
}
