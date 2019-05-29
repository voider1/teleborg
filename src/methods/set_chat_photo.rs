use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to send text messages. On success, the sent `Message` is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SetChatPhoto {
    /// Unique identifier for the target chat.
    pub chat_id: i64,
    /// path of the photo file that should be used.
    #[serde(skip_serializing)]
    pub file: Option<String>,
}

impl_method_multipart!(SetChatPhoto, bool, "setChatPhoto", "photo");
