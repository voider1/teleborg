use super::Method;
use crate::types::{Message, ParseMode, ReplyMarkup};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to send audio files, if you want Telegram clients to display the file as a
/// playable voice message. For this to work, your audio must be in an .ogg file encoded with OPUS
/// (other formats may be sent as Audio or Document). On success, the sent Message is returned.
/// Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendVoice {
    /// Unique identifier for the target chat.
    pub chat_id: i64,
    /// Audio file to send. Pass a file_id as String to send an
    /// audio file that exists on the Telegram servers (recommended),
    /// pass an HTTP URL as a String for Telegram to get an audio file from the Internet,
    /// or upload a new one using multipart/form-data.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing)]
    /// path of the audio file that should be used.
    pub file: Option<String>,
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
    /// Duration of the audio in seconds
    pub duration: Option<i64>,
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

impl_method_multipart!(SendVoice, Message, "sendVoice", "voice");
