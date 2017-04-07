use objects::message::Message;
use objects::user::User;

/// represents an incoming callback query from a callback button in an inline keyboard.
#[derive(Clone, Deserialize, Debug)]
pub struct CallBackQuery {
	id: String,
	from: User,
	message: Option<Message>,
	inline_message_id: Option<String>,
	chat_instance: String,
	data: Option<String>,
	game_short_name: Option<String>,
}