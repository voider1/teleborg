use super::Method;

#[derive(Debug, TypedBuilder, Serialize)]
pub struct DeleteChatPhoto {
    /// Unique identifier for the target chat.
    pub chat_id: i64,
}

impl_method!(DeleteChatPhoto, bool, "deleteChatPhoto");
