use super::Method;

#[derive(Debug, Builder, Serialize)]
pub struct KickChatMember {
    chat_id: i32,
    user_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")] until_date: Option<i32>,
}

impl_builder!(KickChatMember, KickChatMemberBuilder);
impl_method!(KickChatMember, bool, "kickChatMember");
