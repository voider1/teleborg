use crate::types::PollOption;
use serde::Deserialize;

/// This object contains information about a poll.
#[derive(Clone, Deserialize, Debug)]
pub struct Poll {
    /// Unique poll identifier
    pub id: String,
    /// Poll question, 1-255 characters
    pub question: String,
    /// List of poll options
    pub options: Vec<PollOption>,
    /// True, if the poll is closed
    pub is_closed: bool,
}
