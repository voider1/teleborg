use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to generate a new invite link for a chat; any previously generated link will be
/// revoked. The bot must be an administrator in the chat for this to work and must have the
/// appropiate administrator rights. Returns the new invite link on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct ExportChatInviteLink {
    /// Unique identifier for the target chat.
    pub chat_id: i64,
}

impl_method!(ExportChatInviteLink, String, "exportChatInviteLink");
