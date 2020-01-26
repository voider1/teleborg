<<<<<<< HEAD
use crate::types::KeyboardButtonPollType;
=======
>>>>>>> 6c3618d2bbc0e69544e1b98e4b7a197cc6344c01
use serde::{Deserialize, Serialize};

/// This struct represents one button of the `ReplyKeyboard`.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used, it will be sent as a message
    /// when the button is pressed.
    text: String,
    /// If it's supplied with `true`, the user's phone number will be sent as a contact when the
    /// button is pressed. Only available in private chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    request_contact: Option<bool>,
    /// If it's supplied with `true`, the user's current location will be sent when the button is
    /// pressed. Only available in private chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    request_location: Option<bool>,
    /// If specified, the user will be asked to create a poll and send it to the bot when the button is pressed. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    request_poll: Option<KeyboardButtonPollType>,
}
