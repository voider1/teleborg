use serde_json;
use chrono::NaiveDateTime;

use error::{Result, Error};
use user::User;
use chat::Chat;
use message_entity::MessageEntity;
use ::ValueExtension;

#[derive(Debug)]
pub struct Message {
    pub message_id: i64,
    pub from_user: Option<User>,
    pub date: NaiveDateTime,
    pub chat: Chat,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<i64>,
    pub forward_date: Option<NaiveDateTime>,
    pub reply_to_message: Option<Box<Message>>,
    pub edit_date: Option<NaiveDateTime>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>, // TODO: Change to MessageEntity struct
    pub audio: Option<String>, // TODO: Change to audio struct
    pub document: Option<String>, // TODO: Change to Document struct
    pub game: Option<String>, // TODO: Change to Game struct
    pub photo: Option<String>, // TODO: Change to Photo struct
    pub sticker: Option<String>, // TODO: Change to Sticker struct
    pub video: Option<String>, // TODO: Change to Video struct
    pub voice: Option<String>, // TODO: Change to Voice struct
    pub caption: Option<String>,
    pub contact: Option<String>, // TODO: Change to Contact struct
    pub location: Option<String>, // TODO: Change to Location struct
    pub venue: Option<String>, // TODO: Change to Venue struct
    pub new_chat_member: Option<User>, // TODO: Change to User struct
    pub left_chat_member: Option<User>, // TODO: Change to User struct
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<String>, // TODO: Change to array of Photo structs
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,
    pub migrate_to_chat_id: Option<i64>,
    pub migrate_from_chat_id: Option<i64>,
    pub pinned_message: Option<Box<Message>>, // TODO: Change to Message struct
}

impl Message {
    pub fn new(message: &serde_json::Value) -> Result<Message> {
        let message_id = message.as_required_i64("message_id")?;
        let from_user = message.find("from").and_then(|u| User::new(u).ok());
        let date = message.as_required_date("date")?;
        let chat = Chat::new(message.find("chat").ok_or(Error::JsonNotFound)?)?;
        let forward_from = message.find("forward_from").and_then(|u| User::new(u).ok());
        let forward_from_chat = message.find("forward_from_chat").and_then(|fc| Chat::new(fc).ok());
        let forward_from_message_id = message.as_optional_i64("forward_from_message_id");
        let forward_date = message.as_optional_date("forward_date");
        let reply_to_message = message.find("reply_to_message")
            .and_then(|v| Message::new(v).map(|m| m.as_boxed()).ok());
        let edit_date = message.as_optional_date("edit_date");
        let text = message.as_optional_string("text");
        let entities = message.find("entities").and_then(|j| {
            j.as_array().map(|v| v.iter().map(|e| MessageEntity::new(e).unwrap()).collect())
        });
        let audio = None;
        let document = None;
        let game = None;
        let photo = None;
        let sticker = None;
        let video = None;
        let voice = None;
        let caption = message.as_optional_string("caption");
        let contact = None;
        let location = None;
        let venue = None;
        let new_chat_member = message.find("new_chat_member").and_then(|u| User::new(u).ok());
        let left_chat_member = message.find("left_chat_member").and_then(|u| User::new(u).ok());
        let new_chat_title = message.as_optional_string("new_chat_title");
        let new_chat_photo = None;
        let delete_chat_photo = message.as_optional_bool("delete_chat_photo");
        let group_chat_created = message.as_optional_bool("group_chat_created");
        let supergroup_chat_created = message.as_optional_bool("supergroup_chat_created");
        let channel_chat_created = message.as_optional_bool("channel_chat_created");
        let migrate_to_chat_id = message.as_optional_i64("migrate_to_chat_id");
        let migrate_from_chat_id = message.as_optional_i64("migrate_from_chat_id");
        let pinned_message = message.find("pinned_message")
            .and_then(|v| Message::new(v).map(|m| m.as_boxed()).ok());

        Ok(Message {
            message_id: message_id,
            from_user: from_user,
            date: date,
            chat: chat,
            forward_from: forward_from,
            forward_from_chat: forward_from_chat,
            forward_from_message_id: forward_from_message_id,
            forward_date: forward_date,
            reply_to_message: reply_to_message,
            edit_date: edit_date,
            text: text,
            entities: entities,
            audio: audio,
            document: document,
            game: game,
            photo: photo,
            sticker: sticker,
            video: video,
            voice: voice,
            caption: caption,
            contact: contact,
            location: location,
            venue: venue,
            new_chat_member: new_chat_member,
            left_chat_member: left_chat_member,
            new_chat_title: new_chat_title,
            new_chat_photo: new_chat_photo,
            delete_chat_photo: delete_chat_photo,
            group_chat_created: group_chat_created,
            supergroup_chat_created: supergroup_chat_created,
            channel_chat_created: channel_chat_created,
            migrate_to_chat_id: migrate_to_chat_id,
            migrate_from_chat_id: migrate_from_chat_id,
            pinned_message: pinned_message,
        })
    }

    fn as_boxed(self) -> Box<Message> {
        Box::new(self)
    }
}
