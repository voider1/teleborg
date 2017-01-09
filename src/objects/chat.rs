use serde_json;

use error::Result;

#[derive(Clone, Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
    #[serde(rename="type")]
    pub type_chat: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(rename="all_members_are_administrators")]
    pub all_members_are_admins: Option<bool>,
}

impl Chat {
    pub fn new(json: &serde_json::Value) -> Result<Self> {
        let chat: Chat = serde_json::from_value(json.clone())?;
        Ok(chat)
    }
}
