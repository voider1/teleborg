use serde::Deserialize;

/// Represents a Telegram user or bot.
#[derive(Clone, Deserialize, Debug)]
pub struct User {
    /// Unique identifier for this user or bot.
    pub id: i64,
    /// Determines wheter this user is a bot or not.
    pub is_bot: bool,
    /// First name of your user or bot.
    pub first_name: String,
    /// Last name of your user or bot.
    pub last_name: Option<String>,
    /// Username of your user or bot.
    pub username: Option<String>,
    /// IEFT language tag of the user's language.
    pub language_code: Option<String>,
}
