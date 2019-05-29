use super::{Message, User};
use serde::Deserialize;

/// This struct represents an incoming callback query from a callback button in an [inline
/// keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating). If the
/// button that originated the query was attached to a message sent by the bot, the field `message`
/// will be present. If the button was attached to a message sent via the bot (in [inline
/// mode](https://core.telegram.org/bots/api#inline-mode), the field `inline_message_id` will be
/// present. Exactly one of the fields `data` or `game_short_name` will be present.
#[derive(Clone, Deserialize, Debug)]
pub struct CallbackQuery {
    /// Unique identifier for this query.
    pub id: String,
    /// Sender.
    pub from: User,
    /// Message with the callback button that originated the query. Note that message content and
    /// message date will not be available if the message is too old.
    pub message: Option<Message>,
    /// Identifier of the message sent via the bot in inline mode, that originated the query.
    pub inline_message_id: Option<String>,
    /// Global identifier, uniquely corresponding to the chat to which the message with the
    /// callback button was sent. Useful for high scores in
    /// [games](https://core.telegram.org/bots/api#games).
    pub chat_instance: String,
    /// Data associated with the callback button. Be aware that a bad client can send arbitrary
    /// data in this field.
    pub data: Option<String>,
    /// Short name of a `Game` to be returned, serves as the unique identifier for the game.
    pub game_short_name: Option<String>,
}
