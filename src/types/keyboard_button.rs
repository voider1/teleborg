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
}
