use objects::input_message_content::marker::InputMessageContent;

#[derive(Deserialize, Serialize, Debug)]
pub struct InputLocationMessageContent {
    pub latitude: f64,
    pub longitude: f64,
}

impl InputMessageContent for InputLocationMessageContent {}
