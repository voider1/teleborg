use super::Method;

#[derive(Debug, Builder, Serialize)]
pub struct UnbanChatMember {
    chat_id: i32,
    user_id: i32,
}

impl_builder!(UnbanChatMember, UnbanChatMemberBuilder);
impl_method!(UnbanChatMember, bool, "unbanChatMember");
