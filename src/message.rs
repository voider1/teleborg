use serde_json;
use chrono::NaiveDateTime;

use error::Result;
use error::Error;
use user::User;
use chat::Chat;
use ::ValueExtension;

pub struct Message {
    pub message_id: i64,
    pub from_user: Option<User>,  // TODO: Change to User struct
    pub date: NaiveDateTime,
    pub chat: String,  // TODO: Change to Chat struct
    pub forward_from: Option<String>,  // TODO: Change to User struct
    pub forward_from_chat: Option<String>,  // TODO: Change to Chat struct
    pub forward_from_message_id: Option<i32>,
    pub forward_date: Option<i32>,
    pub reply_to_message: Option<Box<Message>>,
    pub edit_date: Option<i32>,
    pub text: Option<String>,
    pub entities: Option<String>,  // TODO: Change to MessageEntity struct
    pub audio: Option<String>,  // TODO: Change to audio struct
    pub document: Option<String>,  // TODO: Change to Document struct
    pub game: Option<String>,  // TODO: Change to Game struct
    pub photo: Option<String>,  // TODO: Change to Photo struct
    pub sticker: Option<String>,  // TODO: Change to Sticker struct
    pub video: Option<String>,  // TODO: Change to Video struct
    pub voice: Option<String>,  // TODO: Change to Voice struct
    pub caption: Option<String>,
    pub contact: Option<String>,  // TODO: Change to Contact struct
    pub location: Option<String>,  // TODO: Change to Location struct
    pub venue: Option<String>,  // TODO: Change to Venue struct
    pub new_chat_member: Option<String>,  // TODO: Change to User struct
    pub left_chat_member: Option<String>,  // TODO: Change to User struct
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<String>,  // TODO: Change to array of Photo structs
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,
    pub migrate_to_chat_id: Option<i32>,
    pub migrate_from_chat_id: Option<i32>,
    pub pinned_message: Option<String>,  // TODO: Change to Message struct
}

impl Message {
    pub fn new(message: &serde_json::Value) -> Result<i32> {
        let message_id = message.as_required_i64("message_id")?;
        let from_user = User::new(message.find("from").ok_or(Error::JsonNotFound)?)?;
        let date = message.as_required_date("date")?;
        let chat = Chat::new(message.find("chat").ok_or(Error::JsonNotFound)?)?;

        Ok(0)
    }
}
