use super::InlineKeyboardButton;

/// Represents a reply keyboard markup.
#[derive(Serialize, Debug)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<InlineKeyboardButton>>,
    pub resize_keyboard: Option<bool>,
    pub one_time_keyboard: Option<bool>,
    pub selective: Option<bool>,
}

impl ReplyKeyboardMarkup {
    /// Constructs a new `ReplyKeyboardMarkup`.
    pub fn new(keyboard: Vec<Vec<InlineKeyboardButton>>,
               resize_keyboard: Option<bool>,
               one_time_keyboard: Option<bool>,
               selective: Option<bool>)
               -> ReplyKeyboardMarkup {
        ReplyKeyboardMarkup {
            keyboard: keyboard,
            resize_keyboard: resize_keyboard,
            one_time_keyboard: one_time_keyboard,
            selective: selective,
        }
    }
}
