use objects::User;

/// Represents a Telegram message entity.
#[derive(Clone, Deserialize, Debug)]
pub struct MessageEntity {
    #[serde(rename="type")]
    type_message_entity: String,
    offset: i64,
    length: i64,
    url: Option<String>,
    user: Option<User>,
}
