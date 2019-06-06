use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to get the number of members in a chat. Returns Int on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct GetChatMembersCount {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: i64,
}

impl_method!(GetChatMembersCount, i64, "getChatMembersCount");