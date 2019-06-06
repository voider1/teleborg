use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to generate a new invite link for a chat; any previously generated link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns the new invite link as String on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct ExportChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
}

impl_method!(ExportChatInviteLink, String, "exportChatInviteLink");