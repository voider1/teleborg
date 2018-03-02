use super::Method;
use types::Message;

#[derive(Debug, Builder, Serialize)]
pub struct ForwardMessage {
    chat_id: i32,
    from_chat_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")] disable_notification: Option<bool>,
    message_id: i32,
}

impl Method for ForwardMessage {
    type Response = Message;
    const PATH: &'static str = "forwardMessage";
}
