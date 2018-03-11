use super::Method;
use types::ChatAction;

#[derive(Debug, Builder, Serialize)]
pub struct SendChatAction {
    chat_id: i64,
    action: ChatAction,
}

impl_builder!(SendChatAction, SendChatActionBuilder);
impl_method!(SendChatAction, bool, "sendChatAction");
