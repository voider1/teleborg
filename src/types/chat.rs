use serde::Deserialize;
use crate::types::{ChatPhoto, Message};

/// This object represents a chat.
#[derive(Clone, Deserialize, Debug)]
pub struct Chat {
    /// Unique identifier for this chat. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Type of chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub kind: String,
    /// Optional. Title, for supergroups, channels and group chats
    pub title: Option<String>,
    /// Optional. Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    /// Optional. First name of the other party in a private chat
    pub first_name: Option<String>,
    /// Optional. Last name of the other party in a private chat
    pub last_name: Option<String>,
    /// Optional. True if a group has ‘All Members Are Admins’ enabled.
    pub all_members_are_administrators: Option<bool>,
    /// Optional. Chat photo. Returned only in getChat.
    pub photo: Option<ChatPhoto>,
    /// Optional. Description, for supergroups and channel chats. Returned only in getChat.
    pub description: Option<String>,
    /// Optional. Chat invite link, for supergroups and channel chats. Each administrator in a chat generates their own invite links, so the bot must first generate the link using exportChatInviteLink. Returned only in getChat.
    pub invite_link: Option<String>,
    /// Optional. Pinned message, for groups, supergroups and channels. Returned only in getChat.
    pub pinned_message: Option<Box<Message>>,
    /// Optional. For supergroups, name of group sticker set. Returned only in getChat.
    pub sticker_set_name: Option<String>,
    /// Optional. True, if the bot can change the group sticker set. Returned only in getChat.
    pub can_set_sticker_set: Option<bool>,
}

