extern crate json;

struct Message {
    pub message_id: i32,
    pub from: Option<String>,  // TODO: Change to User struct
    pub date: i32,
    pub chat: String,  // TODO: Change to Chat struct
    pub forward_from: Option<String>,  // TODO: Change to User struct
    pub forward_from_chat: Option<String>,  // TODO: Change to Chat struct
    pub forward_from_message_id: Option<i32>,
    pub forward_date: Option<i32>,
    pub reply_to_message: Option<Message>,
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
