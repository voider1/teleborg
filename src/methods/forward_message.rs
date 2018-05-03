use super::Method;
use types::Message;

#[derive(Debug, TypedBuilder, Serialize)]
pub struct ForwardMessage {
    chat_id: i32,
    from_chat_id: i32,
    #[default]
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    message_id: i32,
}

impl_method!(ForwardMessage, Message, "forwardMessage");
