use super::Method;
use crate::types::ChatAction;

use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method when you need to tell the user that something is happening on the bot's side.
/// That status is set for 5 seconds or less (when a message from your bot arrives, Telegram
/// clients clear its typing status). Returns `true` on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendChatAction {
    /// Unique identifier for the target chat.
    pub chat_id: i64,
    /// Type of action to broadcast. Use the `ChatAction` enum to specify this.
    pub action: ChatAction,
}

impl_method!(SendChatAction, bool, "sendChatAction");
