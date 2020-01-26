/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use super::Method;
use crate::types::{Message, ReplyMarkup};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to send a game. On success, the sent Message is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendGame {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Short name of the game, serves as the unique identifier for the game. Set up your games via Botfather.
    pub game_short_name: String,
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
    /// A JSON-serialized object for an inline keyboard. If empty, one ‘Play game_title’ button will be shown. If not empty, the first button must launch the game.
    pub reply_markup: Option<ReplyMarkup>,
}

impl_method!(SendGame, Message, "sendGame");
