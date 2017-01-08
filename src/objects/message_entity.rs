use serde_json;

use objects::user::User;
use error::Result;

#[derive(Clone, Deserialize, Debug)]
pub struct MessageEntity {
    #[serde(rename="type")]
    type_message_entity: String,
    offset: i64,
    length: i64,
    url: Option<String>,
    user: Option<User>,
}

impl MessageEntity {
    pub fn new(json: &serde_json::Value) -> Result<Self> {
        let message_entity: MessageEntity = serde_json::from_value(json.clone())?;
        Ok(message_entity)
    }
}
