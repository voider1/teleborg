use super::Method;

#[derive(Debug, Builder, Serialize)]
pub struct DeleteChatPhoto {
    chat_id: i32,
}

impl Method for DeleteChatPhoto {
    type Response = bool;
    const PATH: &'static str = "deleteChatPhoto";
}
