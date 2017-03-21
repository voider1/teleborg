extern crate teleborg;
extern crate reqwest;

#[cfg(test)]
mod tests {
    use teleborg::{Bot, ParseMode};
    use teleborg::objects::{InlineKeyboardButton, InlineKeyboardMarkup};
    use reqwest::Client;

    use std::env;

    const BASE_URL: &'static str = "https://api.telegram.org/bot";

    fn setup() -> (String, i64) {
        let token = env::var("TELEGRAM_BOT_TOKEN")
            .ok()
            .expect("Can't find TELEGRAM_BOT_TOKEN env variable");
        let chat_id = env::var("TELEGRAM_CHAT_ID")
            .ok()
            .expect("Can't find TELEGRAM_CHAT_ID env variable")
            .parse::<i64>()
            .unwrap();
        (token, chat_id)
    }

    #[test]
    fn test_url_inline_keyboard() {
        let (token, chat_id) = setup();
        let bot_url = [BASE_URL, &token].concat();
        let bot = Bot::new(bot_url).unwrap();

        let mut total = Vec::<Vec<InlineKeyboardButton>>::new();
        let mut row = Vec::<InlineKeyboardButton>::new();
        row.push(InlineKeyboardButton::new("TestButton".to_string(), Some("http://github.com/voider1/teleborg".to_string()), None, None, None));
        total.push(row);
        let markup = InlineKeyboardMarkup::new(total);
        let message = bot.send_message(&chat_id, "test", None, None, None, None, Some(&markup));
        assert!(message.is_ok());
    }
}