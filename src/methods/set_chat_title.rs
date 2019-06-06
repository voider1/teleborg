use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SetChatTitle {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// New chat title, 1-255 characters
    pub title: String,
}

impl_method!(SetChatTitle, bool, "setChatTitle");