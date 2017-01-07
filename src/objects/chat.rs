use serde_json;

use error::Result;

#[derive(Clone, Deserialize, Debug)]
pub struct Chat {
    id: i64,
    #[serde(rename="type")]
    type_chat: String,
    title: Option<String>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    #[serde(rename="all_members_are_administrators")]
    all_members_are_admins: Option<bool>,
}

impl Chat {
    pub fn new(json: &serde_json::Value) -> Result<Chat> {
        let chat: Chat = serde_json::from_value(json.clone())?;
        Ok(chat)
    }
}
