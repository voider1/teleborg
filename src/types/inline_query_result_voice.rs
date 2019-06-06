use serde::Deserialize;
use crate::types::{ReplyMarkup, InputMessageContent, ParseMode};

/// Represents a link to a voice recording in an .ogg container encoded with OPUS. By default, this voice recording will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the the voice message.
#[derive(Clone, Deserialize, Debug)]
pub struct InlineQueryResultVoice {
    /// Type of the result, must be voice
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the voice recording
    pub voice_url: String,
    /// Recording title
    pub title: String,
    /// Optional. Caption, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<ParseMode>,
    /// Optional. Recording duration in seconds
    pub voice_duration: Option<i64>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<ReplyMarkup>,
    /// Optional. Content of the message to be sent instead of the voice recording
    pub input_message_content: Option<InputMessageContent>,
}

