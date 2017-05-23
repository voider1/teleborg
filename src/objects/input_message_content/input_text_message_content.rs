use objects::input_message_content::marker::InputMessageContent;
use bot::parse_mode::{ParseMode, get_parse_mode};

#[derive(Deserialize, Serialize, Debug)]
pub struct InputTextMessageContent {
    pub message_text: String,
    pub parse_mode: Option<String>,
    pub disable_web_page_preview: Option<bool>,
}

impl InputTextMessageContent {
    pub fn new(message_text: String, parse_mode: Option<ParseMode>, disable_web_page_preview: Option<bool>) -> Self {
        InputTextMessageContent {
            message_text: message_text,
            parse_mode: Some(get_parse_mode(&parse_mode.unwrap_or(ParseMode::Text))),
            disable_web_page_preview: disable_web_page_preview,
        }
    }
}

impl InputMessageContent for InputTextMessageContent {}
