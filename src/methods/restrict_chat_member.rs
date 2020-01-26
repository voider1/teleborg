/// This code is generated using teleborg-api-validator (https://gitlab.com/b.wisman155/teleborg-api-validater)
use super::Method;
use crate::types::ChatPermissions;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate admin rights. Pass True for all permissions to lift restrictions from a user. Returns True on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct RestrictChatMember {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: i64,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// New user permissions
    pub permissions: ChatPermissions,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date when restrictions will be lifted for the user, unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    pub until_date: Option<i64>,
}

impl_method!(RestrictChatMember, bool, "restrictChatMember");
