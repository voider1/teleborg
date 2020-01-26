/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use super::Method;
use crate::types::{ChatAction};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// We only recommend using this method when a response from the bot will take a noticeable amount of time to arrive.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendChatAction {
    /// Unique identifier for the target chat or username of the target channel (in the
    pub chat_id: i64,
    /// Type of action to broadcast. Choose one, depending on what the user is about to
    pub action: ChatAction,
}

impl_method!(SendChatAction, bool);
