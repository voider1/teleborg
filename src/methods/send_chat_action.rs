/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use super::Method;
use crate::types::ChatAction;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// We only recommend using this method when a response from the bot will take a noticeable amount of time to arrive.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendChatAction {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Type of action to broadcast. Choose one, depending on what the user is about to receive: typing for text messages, upload_photo for photos, record_video or upload_video for videos, record_audio or upload_audio for audio files, upload_document for general files, find_location for location data, record_video_note or upload_video_note for video notes.
    pub action: ChatAction,
}

impl_method!(SendChatAction, bool, "sendChatAction");
