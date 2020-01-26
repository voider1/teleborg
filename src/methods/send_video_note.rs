/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use super::Method;
use crate::types::{ReplyMarkup, Message};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// As of v.4.0, Telegram clients support rounded square mp4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent Message is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendVideoNote {
    /// Unique identifier for the target chat or username of the target channel (in the
    pub chat_id: i64,
    /// Video note to send. Pass a file_id as String to send a video note that exists on
    pub video_note: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Duration of sent video in seconds
    pub duration: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Video width and height, i.e. diameter of the video message
    pub length: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file
    pub thumb: String,
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

impl_method!(SendVideoNote, Message, video_note, thumb);
