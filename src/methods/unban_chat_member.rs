use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to unban a previously kicked user in a supergroup or channel. The user will not return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. Returns True on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct UnbanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or
    pub chat_id: i64,
    /// Unique identifier of the target user
    pub user_id: i64,
}

impl_method!(UnbanChatMember, bool);
