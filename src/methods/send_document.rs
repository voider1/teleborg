use super::Method;
use crate::types::{Message, ParseMode, ReplyMarkup};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to send general files. On success, the sent Message is returned.
/// Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendDocument {
    /// Unique identifier for the target chat.
    pub chat_id: i64,
    /// File to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended),
    ///	pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing)]
    /// path of the photo file that should be used.
    pub file: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is
    /// supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail‘s width and height should not exceed 90. Ignored if the file is not uploaded
    /// using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file
    pub thumb: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// path of the thumbnail file that should be used.
    pub thumb_file: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption text for the photo.
    pub caption: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Select the way your message should be parsed using the `ParseMode` struct.
    pub parse_mode: Option<ParseMode>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users get a notification without sound.
    pub disable_notification: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message.
    pub reply_to_message_id: Option<i32>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Interface options.
    pub reply_markup: Option<ReplyMarkup>,
}

impl_method_multipart_thumb!(SendDocument, Message, "sendDocument", "document");
