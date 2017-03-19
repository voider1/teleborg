/// Represents a Telegram user.
#[derive(Clone, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}
