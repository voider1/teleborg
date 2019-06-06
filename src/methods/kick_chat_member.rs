use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to kick a user from a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the group on their own using invite links, etc., unless unbanned first. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct KickChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: i64,
    /// Unique identifier of the target user
    pub user_id: i64,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date when the user will be unbanned, unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever
    pub until_date: Option<i64>,
}

impl_method!(KickChatMember, bool, "kickChatMember");