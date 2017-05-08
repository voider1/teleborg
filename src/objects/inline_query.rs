use objects::Location;

/// Represents a Telegram inline_query.
#[derive(Clone, Deserialize, Debug)]
pub struct InlineQuery {
    id: i64,
    from: User,
    location: Option<Location>,
    query: String,
    offset: String,
}