use super::Method;

#[derive(Debug, TypedBuilder, Serialize)]
pub struct ExportChatInviteLink {
    chat_id: i32,
}

impl_method!(ExportChatInviteLink, String, "exportChatInviteLink");
