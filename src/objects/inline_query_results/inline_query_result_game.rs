use uuid::Uuid;

use objects::inline_query_results::InlineQueryResult;
use objects::inline_keyboard::InlineKeyboardMarkup;

use super::InlineQueryResultType;

use std::any::Any;

/// Represents a Game.
/// The game must be created on your bot to be able to use it with `InlineQueryResultGame`.
/// Do this by doing `/newgame` on telegrams `BotFather` and selecting your bot as host.
#[derive(Serialize)]
pub struct InlineQueryResultGame {
    #[serde(rename="type")]
    result_type: String,
    id: String,
    game_short_name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl InlineQueryResultGame {
    pub fn new(game_short_name: String,
               reply_markup: Option<InlineKeyboardMarkup>,)
               -> Self {
        let result_type = "game".to_string();
        let id = format!("{}", Uuid::new_v4());

        InlineQueryResultGame {
            result_type: result_type,
            id: id,
            game_short_name: game_short_name,
            reply_markup: reply_markup,
        }
    }
}

impl InlineQueryResult for InlineQueryResultGame {
    fn as_any(&self) -> &Any {
        self
    }

    fn get_type(&self) -> InlineQueryResultType {
        InlineQueryResultType::Game
    }
}
