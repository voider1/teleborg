use crate::types::PollOption;
/// This code is generated using teleborg-api-validator (https://gitlab.com/b.wisman155/teleborg-api-validater)
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
    /// Total number of users that voted in the poll
    pub total_voter_count: i64,
    /// True, if the poll is closed
    pub is_closed: bool,
    /// True, if the poll is anonymous
    pub is_anonymous: bool,
    /// Poll type, currently can be “regular” or “quiz”
    #[serde(rename = "type")]
    pub kind: String,
    /// True, if the poll allows multiple answers
    pub allows_multiple_answers: bool,
    /// Optional. 0-based identifier of the correct answer option. Available only for polls in the quiz mode, which are closed, or was sent (not forwarded) by the bot or to the private chat with the bot.
    pub correct_option_id: Option<i64>,
}
