use super::Method;
use crate::types::ChatMember;
use serde::Serialize;
use typed_builder::TypedBuilder;

type Administrators = Vec<ChatMember>;

/// Use this method to get a list of administrators in a chat. On success, returns an Array of
/// ChatMember objects that contains information about all chat administrators except other bots.
/// If the chat is a group or a supergroup and no administrators were appointed, only the creator
/// will be returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct GetChatAdministrators {
    /// Unique identifier for the target chat.
    pub chat_id: i64,
}

impl_method!(
    GetChatAdministrators,
    Administrators,
    "getChatAdministrators"
);
