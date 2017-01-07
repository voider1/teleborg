use serde_derive;
use serde_json;

use error::{Error, Result};

#[derive(Clone, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

impl User {
    pub fn new(json: &serde_json::Value) -> Result<User> {
        let user: User = serde_json::from_value(json.clone())?;
        Ok(user)
    }
}
