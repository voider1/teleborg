use super::InlineKeyboardButton;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup {
        inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
    },
    ReplyKeyboardMarkup {
        keyboard: Vec<Vec<InlineKeyboardButton>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        resize_keyboard: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        one_time_keyboard: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        selective: Option<bool>,
    },
    ReplyKeyboardRemove {
        remove_keyboard: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        selective: Option<bool>,
    },
    ForceReply {
        force_reply: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        selective: Option<bool>,
    },
}
