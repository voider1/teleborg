use objects::PhotoSize;
use objects::MessageEntity;
use objects::Animation;

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
