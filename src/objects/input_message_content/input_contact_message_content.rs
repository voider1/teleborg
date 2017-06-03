use super::{InputMessageContent, InputMessageType};

use std::any::Any;

#[derive(Serialize, Deserialize)]
pub struct InputContactMessageContent {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
}

impl InputContactMessageContent {
    pub fn new(phone_number: String, first_name: String, last_name: Option<String>) -> Self {
        InputContactMessageContent{
            phone_number: phone_number,
            first_name: first_name,
            last_name: last_name,
        }
    }
}

impl InputMessageContent for InputContactMessageContent {
    fn as_any(&self) -> &Any {
        self
    }

    fn get_type(&self) -> InputMessageType {
        InputMessageType::Contact
    }
}
