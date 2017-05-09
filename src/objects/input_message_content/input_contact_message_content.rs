use objects::input_message_content::marker::InputMessageContent;

pub struct InputContactMessageContent {
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
}

impl InputMessageContent for InputContactMessageContent {}