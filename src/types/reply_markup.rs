use super::{InlineKeyboardButton, KeyboardButton};

/// This enum represents all the possible reply markups.
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ReplyMarkup {
    /// This variant represents an [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating)
    /// which will appear right next to the message to which it belongs.
    InlineKeyboardMarkup {
        /// Vector of button rows, each represented by an vector of `InlineKeyboardButton`
        /// instances.
        inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
    },
    /// This variant represents a [custom keyboard](https://core.telegram.org/bots#keyboards)
    /// with reply options.
    ReplyKeyboardMarkup {
        /// Vector of button rows, each represented by a vector of `KeyboardButton` instances.
        keyboard: Vec<Vec<KeyboardButton>>,
        /// If it's supplied with `true`, requests clients to resize the keyboard vertically for an optimal fit.
        #[serde(skip_serializing_if = "Option::is_none")]
        resize_keyboard: Option<bool>,
        /// If it's supplied with `true`, requests clients to hide the keyboard as soon as it's
        /// been used. The keyboard will still be available, but clients will automatically display
        /// the usual letter-keyboard in the chat - the user can press a special button in the
        /// input field to see the custom keyboard again.
        #[serde(skip_serializing_if = "Option::is_none")]
        one_time_keyboard: Option<bool>,
        /// If it's supplied with `true`, it will show the keyboard only to specific users which
        /// are mentioned in the text or to the (if supplied) `reply_to_message_id`.
        #[serde(skip_serializing_if = "Option::is_none")]
        selective: Option<bool>,
    },
    /// When receiving a message with this variant, Telegram clients will remove the current custom
    /// keyboard and display the default letter-keyboard. By default, custom keyboards are
    /// displayed until a new keyboard is sent by a bot. An exception is made for one-time
    /// keyboards which are hidden automatically after the user presses a button (see
    /// `ReplyKeyboardMarkup`).
    ReplyKeyboardRemove {
        /// If supplied with `true`, requests clients to remove the custom keyboard (users will not
        /// be able to summon the keyboard); if you want to hide the keyboard from sight, but keep
        /// it accessible, use `one_time_keyboard` in `ReplyKeyboardMarkup`.
        remove_keyboard: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        /// If it's supplied with `true`, it will show the keyboard only to specific users which
        /// are mentioned in the text or to the (if supplied) `reply_to_message_id`.
        selective: Option<bool>,
    },
    /// When receiving a message with this variant, Telegram clients will display a reply interface
    /// to the user (act as if the user has selected the bot's message and tapped "Reply"). This
    /// can be extremely useful if you want to create user-friendly step-by-step interfaces without
    /// having to sacrifice [privacy mode](https://core.telegram.org/bots#privacy-mode).
    ForceReply {
        /// Shows the reply interface to the user, as if they manually selected the bot's message
        /// and tapped "Reply".
        force_reply: bool,
        /// If it's supplied with `true`, it will show the keyboard only to specific users which
        /// are mentioned in the text or to the (if supplied) `reply_to_message_id`.
        #[serde(skip_serializing_if = "Option::is_none")]
        selective: Option<bool>,
    },
}
