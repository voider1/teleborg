use super::Method;
use crate::types::{InputMedia, ReplyMarkup, Message};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to edit animation, audio, document, photo, or video messages. If a message is a part of a message album, then it can be edited only to a photo or a video. Otherwise, message type can be changed arbitrarily. When inline message is edited, new file can't be uploaded. Use previously uploaded file via its file_id or specify a URL. On success, if the edited message was sent by the bot, the edited Message is returned, otherwise True is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct EditMessageMedia {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    pub message_id: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for a new media content of the message
    pub media: InputMedia,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for a new inline keyboard.
    pub reply_markup: Option<ReplyMarkup>,
}

impl_method!(EditMessageMedia, Message);
