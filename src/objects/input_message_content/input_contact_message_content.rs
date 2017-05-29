use objects::input_message_content::marker::InputMessageContent;

#[derive(Serialize, Deserialize, Debug)]
pub struct InputContactMessageContent {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
}

impl InputMessageContent for InputContactMessageContent {}