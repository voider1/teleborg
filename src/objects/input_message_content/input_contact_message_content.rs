use super::{InputMessageContent, InputMessageContentType};

use std::any::Any;

/// Represents the content of a contact message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize)]
pub struct InputContactMessageContent {
    pub phone_number: String,
    pub first_name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_name: Option<String>,
}

impl InputContactMessageContent {
    pub fn new(phone_number: String, first_name: String, last_name: Option<String>) -> Self {
        InputContactMessageContent {
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

    fn get_type(&self) -> InputMessageContentType {
        InputMessageContentType::Contact
    }
}
