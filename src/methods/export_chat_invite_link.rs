use super::Method;

#[derive(Debug, Builder, Serialize)]
pub struct ExportChatInviteLink {
    chat_id: i32,
}

impl Method for ExportChatInviteLink {
    type Response = String;
    const PATH: &'static str = "exportChatInviteLink";
}
