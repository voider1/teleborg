use objects::{User, Location};

/// Represents a Telegram inline_query.
#[derive(Clone, Deserialize, Debug)]
pub struct InlineQuery {
    pub id: i64,
    pub from: User,
    pub location: Option<Location>,
    pub query: String,
    pub offset: String,
}