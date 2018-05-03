use super::Method;

#[derive(Debug, TypedBuilder, Serialize)]
pub struct DeleteChatPhoto {
    chat_id: i32,
}

impl_method!(DeleteChatPhoto, bool, "deleteChatPhoto");
