/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use super::Method;
use crate::types::Message;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to set the score of the specified user in a game. On success, if the message was sent by the bot, returns the edited Message, otherwise returns True. Returns an error, if the new score is not greater than the user's current score in the chat and force is False.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SetGameScore {
    /// User identifier
    pub user_id: i64,
    /// New score, must be non-negative
    pub score: i64,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True, if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
    pub force: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True, if the game message should not be automatically edited to include the current scoreboard
    pub disable_edit_message: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    pub chat_id: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Identifier of the sent message
    pub message_id: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
}

impl_method!(SetGameScore, Message, "setGameScore");
