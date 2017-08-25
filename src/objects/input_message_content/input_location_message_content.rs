use super::{InputMessageContent, InputMessageContentType};

use std::any::Any;

/// Represents the content of a location message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize)]
pub struct InputLocationMessageContent {
    pub latitude: f64,
    pub longitude: f64,
}

impl InputLocationMessageContent {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        InputLocationMessageContent {
            latitude: latitude,
            longitude: longitude,
        }
    }
}

impl InputMessageContent for InputLocationMessageContent {
    fn as_any(&self) -> &Any {
        self
    }

    fn get_type(&self) -> InputMessageContentType {
        InputMessageContentType::Location
    }
}
