/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use super::Method;
use crate::types::{ChatMember};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to get a list of administrators in a chat. On success, returns an Array of ChatMember objects that contains information about all chat administrators except other bots. If the chat is a group or a supergroup and no administrators were appointed, only the creator will be returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct GetChatAdministrators {
    /// Unique identifier for the target chat or username of the target supergroup or
    pub chat_id: i64,
}

impl_method!(GetChatAdministrators, Vec<ChatMember>);
