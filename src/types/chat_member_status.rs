use serde::Deserialize;

/// The status for a chat member.
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ChatMemberStatus {
    /// Chat member status is Creator.
    Creator,
    /// Chat member status is Administrator.
    Administrator,
    /// Chat member status is Member.
    Member,
    /// Chat member status is Restricted.
    Restricted,
    /// Chat member status is Left.
    Left,
    /// Chat member status is Kicked.
    Kicked,
}
