use super::Method;

use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to kick a user from a group, supergroup or channel. In the case of supergroups
/// and channels the user will not be able to return to the group on their own using invite links,
/// etc., unless unbanned first. The bot must be an administrator in the chat for this to work and
/// must have the appropiate administrator rights. Returns `true` on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct KickChatMember {
    chat_id: i32,
    user_id: i32,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<i32>,
}

impl_method!(KickChatMember, bool, "kickChatMember");
