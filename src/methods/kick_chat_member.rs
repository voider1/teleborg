use super::Method;

#[derive(Debug, TypedBuilder, Serialize)]
pub struct KickChatMember {
    chat_id: i32,
    user_id: i32,
    #[default]
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<i32>,
}

impl_method!(KickChatMember, bool, "kickChatMember");
