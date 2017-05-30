use objects::input_message_content::marker::InputMessageContent;

#[derive(Serialize, Deserialize, Debug)]
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

impl InputMessageContent for InputLocationMessageContent {}
