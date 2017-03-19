/// Represents a Telegram chat.
#[derive(Clone, Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
    #[serde(rename="type")]
    pub kind: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(rename="all_members_are_administrators")]
    pub all_members_are_admins: Option<bool>,
}
