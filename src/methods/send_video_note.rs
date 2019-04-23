use super::Method;
use crate::types::{Message, ReplyMarkup};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// As of v.4.0, Telegram clients support rounded square mp4 videos of up to 1 minute long.
/// Use this method to send video messages. On success, the sent Message is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendVideoNote {
    /// Unique identifier for the target chat.
    pub chat_id: i64,
    /// Video note to send. Pass a file_id as String to send a video note that exists on the
    /// Telegram servers (recommended) or upload a new video using multipart/form-data. More info
    /// on Sending Files ». Sending video notes by a URL is currently unsupported.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing)]
    /// path of the video note file that should be used.
    pub file: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Duration of the video in seconds
    pub duration: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Video width and height, i.e. diameter of the video message
    pub length: Option<i64>,
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

impl_method_multipart_thumb!(SendVideoNote, Message, "sendVideoNote", "video_note");