use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method for your bot to leave a group, supergroup or channel. Returns True on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct LeaveChat {
    /// Unique identifier for the target chat.
    pub chat_id: i64,
}

impl_method!(LeaveChat, bool, "leaveChat");
