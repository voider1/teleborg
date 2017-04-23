use objects::PhotoSize;
use objects::MessageEntity;
use objects::Animation;

/// Represents a venue
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Game {
    title: String,
    description: String,
    photo: Vec<PhotoSize>,
    text: Option<String>,
    text_entities: Vec<MessageEntity>,
    animation: Option<Animation>,
}
