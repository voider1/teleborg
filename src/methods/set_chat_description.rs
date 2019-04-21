use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to change the description of a supergroup or a channel.
/// The bot must be an administrator in the chat for this to work and must have the appropriate admin rights.
/// Returns True on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SetChatDescription {
    /// Unique identifier for the target chat.
    pub chat_id: i64,
    #[builder(default)]
    /// New chat description, 0-255 characters
    pub description: Option<String>,
}

impl_method!(SetChatDescription, bool, "setChatDescription");
