use super::Method;
use types::ChatAction;

#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendChatAction {
    chat_id: i64,
    action: ChatAction,
}

impl_method!(SendChatAction, bool, "sendChatAction");
