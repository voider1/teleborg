use serde::Deserialize;
use crate::types::{ParseMode};

/// Represents the content of a text message to be sent as the result of an inline query.
#[derive(Clone, Deserialize, Debug)]
pub struct InputTextMessageContent {
    /// Text of the message to be sent, 1-4096 characters
    pub message_text: String,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in your bot's message.
    pub parse_mode: Option<ParseMode>,
    /// Optional. Disables link previews for links in the sent message
    pub disable_web_page_preview: Option<bool>,
}

