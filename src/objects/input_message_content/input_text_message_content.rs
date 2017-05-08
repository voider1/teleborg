use bot::parse_mode::ParseMode;

pub struct InputTextMessageContent {
    pub message_text: String,
    pub parse_mode: Option<ParseMode>,
    pub disable_web_page_preview: Option<bool>,
}