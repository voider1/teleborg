use super::Method;
use crate::types::{Chat};use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to get up to date information about the chat (current name of the user for one-on-one conversations, current username of a user, group or channel, etc.). Returns a Chat object on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct GetChat {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: i64,
}

impl_method!(GetChat, Chat, "getChat");