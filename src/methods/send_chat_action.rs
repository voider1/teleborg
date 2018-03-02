use super::Method;

#[derive(Debug, Builder, Serialize)]
pub struct SendChatAction {
    chat_id: i32,
    action: &'static str,
}

impl Method for SendChatAction {
    type Response = bool;
    const PATH: &'static str = "sendChatAction";
}
