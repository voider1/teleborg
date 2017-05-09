use objects::input_message_content::marker::InputMessageContent;

#[derive(Deserialize, Debug)]
pub struct InputTextMessageContent {
    pub message_text: String,
    pub parse_mode: Option<String>,
    pub disable_web_page_preview: Option<bool>,
}

impl InputMessageContent for InputTextMessageContent {}