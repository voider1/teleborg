use super::Method;

#[derive(Debug, Builder, Serialize)]
pub struct UnbanChatMember {
    chat_id: i32,
    user_id: i32,
}

impl Method for UnbanChatMember {
    type Response = bool;
    const PATH: &'static str = "unbanChatMember";
}
