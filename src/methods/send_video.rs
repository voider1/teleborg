/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use super::Method;
use crate::types::{ParseMode, ReplyMarkup, Message};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to send video files, Telegram clients support mp4 videos (other formats may be sent as Document). On success, the sent Message is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendVideo {
    /// Unique identifier for the target chat or username of the target channel (in the
    pub chat_id: i64,
    /// Video to send. Pass a file_id as String to send a video that exists on the
    pub video: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Duration of sent video in seconds
    pub duration: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Video width
    pub width: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Video height
    pub height: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file
    pub thumb: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Video caption (may also be used when resending videos by file_id), 0-1024
    pub caption: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-
    pub parse_mode: Option<ParseMode>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True, if the uploaded video is suitable for streaming
    pub supports_streaming: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional interface options. A JSON-serialized object for an inline keyboard,
    pub reply_markup: Option<ReplyMarkup>,
}

impl_method!(SendVideo, Message, video, thumb);
