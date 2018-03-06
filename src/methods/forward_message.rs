use super::Method;
use types::Message;

#[derive(Debug, Builder, Serialize)]
pub struct ForwardMessage {
    chat_id: i32,
    from_chat_id: i32,
    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    message_id: i32,
}

impl_builder!(ForwardMessage, ForwardMessageBuilder);
impl_method!(ForwardMessage, Message, "forwardMessage");
