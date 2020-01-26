use crate::types::User;
/// This code is generated using teleborg-api-validator (https://gitlab.com/b.wisman155/teleborg-api-validater)
use serde::Deserialize;

/// This object represents one row of the high scores table for a game.
#[derive(Clone, Deserialize, Debug)]
pub struct GameHighScore {
    /// Position in high score table for the game
    pub position: i64,
    /// User
    pub user: User,
    /// Score
    pub score: i64,
}
