use serde::Deserialize;
use crate::types::{User};

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

