use serde::Deserialize;
use crate::types::{User, Location};

/// This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
#[derive(Clone, Deserialize, Debug)]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Optional. Sender location, only for bots that request user location
    pub location: Option<Location>,
    /// Text of the query (up to 512 characters)
    pub query: String,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: String,
}

