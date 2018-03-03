use super::Method;

#[derive(Debug, Builder, Serialize)]
pub struct SendChatAction {
    chat_id: i32,
    action: &'static str,
}

impl_builder!(SendChatAction, SendChatActionBuilder);
impl_method!(SendChatAction, bool, "sendChatAction");
