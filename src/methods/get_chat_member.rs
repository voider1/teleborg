use super::Method;
use crate::types::ChatMember;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to get information about a member of a chat. Returns a ChatMember object on
/// success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct GetChatMember {
    /// Unique identifier for the target chat.
    pub chat_id: i64,
    /// Unique identifier of the target user
    pub user_id: i64,
}

impl_method!(GetChatMember, ChatMember, "getChatMember");
