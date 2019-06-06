use serde::Deserialize;
use crate::types::{ReplyMarkup, InputMessageContent, ParseMode};

/// Represents a link to a voice message stored on the Telegram servers. By default, this voice message will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the voice message.
#[derive(Clone, Deserialize, Debug)]
pub struct InlineQueryResultCachedVoice {
    /// Type of the result, must be voice
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the voice message
    pub voice_file_id: String,
    /// Voice message title
    pub title: String,
    /// Optional. Caption, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<ParseMode>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<ReplyMarkup>,
    /// Optional. Content of the message to be sent instead of the voice message
    pub input_message_content: Option<InputMessageContent>,
}

