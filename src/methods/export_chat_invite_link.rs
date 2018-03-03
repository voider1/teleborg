use super::Method;

#[derive(Debug, Builder, Serialize)]
pub struct ExportChatInviteLink {
    chat_id: i32,
}

impl_builder!(ExportChatInviteLink, ExportChatInviteLinkBuilder);
impl_method!(ExportChatInviteLink, String, "exportChatInviteLink");
