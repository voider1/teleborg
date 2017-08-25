use super::{InputMessageContent, InputMessageContentType};
use bot::parse_mode::{ParseMode, get_parse_mode};

use std::any::Any;

/// Represents the content of a text message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize)]
pub struct InputTextMessageContent {
    pub message_text: String,
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
}

impl InputTextMessageContent {
    pub fn new(message_text: String,
               parse_mode: Option<ParseMode>,
               disable_web_page_preview: Option<bool>)
               -> Self {
        let parse_mode = Some(get_parse_mode(&parse_mode.unwrap_or(ParseMode::Text)));
        InputTextMessageContent {
            message_text: message_text,
            parse_mode: parse_mode,
            disable_web_page_preview: disable_web_page_preview,
        }
    }
}

impl InputMessageContent for InputTextMessageContent {
    fn as_any(&self) -> &Any {
        self
    }

    fn get_type(&self) -> InputMessageContentType {
        InputMessageContentType::Text
    }
}
