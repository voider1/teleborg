use super::Method;
use crate::types::ChatPermissions;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// This code is generated using teleborg-api-validator
/// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the can_restrict_members admin rights. Returns True on success.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SetChatPermissions {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: i64,
    /// New default chat permissions
    pub permissions: ChatPermissions,
}

impl_method!(SetChatPermissions, bool, "setChatPermissions");
