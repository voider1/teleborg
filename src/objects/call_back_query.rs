use objects::message::Message;
use objects::user::User;

/// Represents an incoming callback query from a callback button in an inline keyboard.
/// If the button that originated the query was attached to a message sent by the bot,
/// the field message will be present.
/// If the button was attached to a message sent via the bot (in inline mode),
/// the field inline_message_id will be present.
/// Exactly one of the fields data or game_short_name will be present.
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
