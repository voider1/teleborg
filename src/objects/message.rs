use objects::user::User;
use objects::chat::Chat;
use objects::message_entity::MessageEntity;

/// Represents a Telegram message.
#[derive(Clone, Deserialize, Debug)]
pub struct Message {
    pub message_id: i64,
    pub from: Option<User>,
    pub date: i64,
    pub chat: Chat,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<i64>,
    pub forward_date: Option<i64>,
    pub reply_to_message: Option<Box<Message>>,
    pub edit_date: Option<i64>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub audio: Option<String>,
    pub document: Option<String>,
    pub game: Option<String>,
    pub photo: Option<String>,
    pub sticker: Option<String>,
    pub video: Option<String>,
    pub voice: Option<String>,
    pub caption: Option<String>,
    pub contact: Option<String>,
    pub location: Option<String>,
    pub venue: Option<String>,
    pub new_chat_member: Option<User>,
    pub left_chat_member: Option<User>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<String>,
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,
    pub migrate_to_chat_id: Option<i64>,
    pub migrate_from_chat_id: Option<i64>,
    pub pinned_message: Option<Box<Message>>,
}
