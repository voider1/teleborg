use super::Method;

#[derive(Debug, TypedBuilder, Serialize)]
pub struct UnbanChatMember {
    chat_id: i32,
    user_id: i32,
}

impl_method!(UnbanChatMember, bool, "unbanChatMember");
