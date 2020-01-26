use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns True on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SetChatAdministratorCustomTitle {
    /// Unique identifier for the target chat or username of the target supergroup (in
    pub chat_id: i64,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// New custom title for the administrator; 0-16 characters, emoji are not allowed
    pub custom_title: String,
}

impl_method!(SetChatAdministratorCustomTitle, bool);
