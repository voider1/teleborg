use serde::Deserialize;

/// This object contains information about one answer option in a poll.
#[derive(Clone, Deserialize, Debug)]
pub struct PollOption {
    /// Option text, 1-100 characters
    pub text: String,
    /// Number of users that voted for this option
    pub voter_count: i64,
}

