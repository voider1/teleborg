use super::{ChatPhoto, Message};
use serde::Deserialize;

/// This struct represents a chat.
#[derive(Clone, Deserialize, Debug)]
pub struct Chat {
    /// Unique identifier for this chat.
    pub id: i64,
    /// Type of chat, can be either "private", "group", "supergroup" or "channel".
    /// Called a "kind", because "type" is a reserved keyword.
    #[serde(rename = "type")]
    pub kind: String,
    /// Title for supergroups, channels and group chats.
    pub title: Option<String>,
    /// Username for private chats, supergroups and channels if available.
    pub username: Option<String>,
    /// First name of the other party in a private chat.
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat.
    pub last_name: Option<String>,
    /// True if a group has "All Members are Admins" enabled.
    pub all_members_are_administrators: Option<bool>,
    /// Chat photo. Returned only in `GetChat`.
    pub photo: Option<ChatPhoto>,
    /// Description for supergroups and channel chats. Returned only in `GetChat`.
    pub description: Option<String>,
    /// Chat invite link for supergroups and channel chats. Returned only in `GetChat`.
    pub invite_link: Option<String>,
    /// Pinned message for supergroups and channel chats. Returned only in `GetChat`.
    pub pinned_message: Option<Box<Message>>,
    /// For supergroups, name of the group sticker set. Returned only in `GetChat`.
    pub sticker_set_name: Option<String>,
    /// True if the bot can change the group sticker set. Returned only in `GetChat`.
    pub can_set_sticker_set: Option<bool>,
}
