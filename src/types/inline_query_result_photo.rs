use serde::Deserialize;
use crate::types::{ReplyMarkup, InputMessageContent, ParseMode};

/// Represents a link to a photo. By default, this photo will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the photo.
#[derive(Clone, Deserialize, Debug)]
pub struct InlineQueryResultPhoto {
    /// Type of the result, must be photo
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL of the photo. Photo must be in jpeg format. Photo size must not exceed 5MB
    pub photo_url: String,
    /// URL of the thumbnail for the photo
    pub thumb_url: String,
    /// Optional. Width of the photo
    pub photo_width: Option<i64>,
    /// Optional. Height of the photo
    pub photo_height: Option<i64>,
    /// Optional. Title for the result
    pub title: Option<String>,
    /// Optional. Short description of the result
    pub description: Option<String>,
    /// Optional. Caption of the photo to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<ParseMode>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<ReplyMarkup>,
    /// Optional. Content of the message to be sent instead of the photo
    pub input_message_content: Option<InputMessageContent>,
}

