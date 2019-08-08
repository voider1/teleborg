use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SetChatPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    #[builder(default)]
    /// photo file to send with multipart
    pub photo: String,
}

impl_method!(SetChatPhoto, bool, photo);
