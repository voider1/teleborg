use objects::message::Message;
use objects::user::User;

#[derive(Clone, Deserialize, Debug)]
pub struct CallBackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}