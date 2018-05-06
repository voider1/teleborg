use super::Method;
use types::Message;

/// Use this method to forward messages of any kind. On success, the sent `Message` is returned.
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
