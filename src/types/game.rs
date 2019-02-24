use super::{Animation, MessageEntity, PhotoSize};

use serde::Deserialize;

/// Represents a game.
#[derive(Clone, Deserialize, Debug)]
pub struct Game {
    title: String,
    description: String,
    photo: Vec<PhotoSize>,
    text: Option<String>,
    text_entities: Option<Vec<MessageEntity>>,
    animation: Option<Animation>,
}
