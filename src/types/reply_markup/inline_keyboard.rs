use super::InlineKeyboardButton;
use marker::ReplyMarkup;

/// Represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Debug, Default, Serialize)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    /// Constructs a new `InlineKeyboardMarkup`.
    pub fn new(inline_keyboard: Vec<Vec<InlineKeyboardButton>>) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup { inline_keyboard }
    }
}

impl ReplyMarkup for InlineKeyboardMarkup {}
