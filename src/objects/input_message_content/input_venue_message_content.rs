use objects::input_message_content::marker::InputMessageContent;

#[derive(Clone, Deserialize, Debug)]
pub struct InputVenueMessageContent {
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
}

impl InputMessageContent for InputVenueMessageContent {}