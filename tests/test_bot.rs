extern crate teleborg;
extern crate reqwest;

#[cfg(test)]
mod tests {
    use teleborg::{Bot, ParseMode, ChatAction};
    use teleborg::objects::{Contact, InlineKeyboardButton, InlineKeyboardMarkup};
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
    fn test_get_me() {
        let (token, _) = setup();
        let bot_url = [BASE_URL, &token].concat();
        let client = Client::new().unwrap();
        let json = Bot::get_me(&client, &bot_url);
        assert!(json.is_ok());
    }

    #[test]
    fn test_send_message() {
        let (token, chat_id) = setup();
        let bot_url = [BASE_URL, &token].concat();
        let bot = Bot::new(bot_url).unwrap();
        let message = bot.send_message(&chat_id, "test", None, None, None, None, None);
        assert!(message.is_ok());
    }

    #[test]
    fn test_send_text_message() {
        let (token, chat_id) = setup();
        let bot_url = [BASE_URL, &token].concat();
        let bot = Bot::new(bot_url).unwrap();
        let message = bot.send_message(&chat_id,
                                       "test",
                                       Some(&ParseMode::Text),
                                       None,
                                       None,
                                       None,
                                       None);
        assert!(message.is_ok());
    }

    #[test]
    fn test_send_markdown_message() {
        let (token, chat_id) = setup();
        let bot_url = [BASE_URL, &token].concat();
        let bot = Bot::new(bot_url).unwrap();
        let message = bot.send_message(&chat_id,
                                       "*test*",
                                       Some(&ParseMode::Markdown),
                                       None,
                                       None,
                                       None,
                                       None);
        assert!(message.is_ok());
    }

    #[test]
    fn test_send_html_message() {
        let (token, chat_id) = setup();
        let bot_url = [BASE_URL, &token].concat();
        let bot = Bot::new(bot_url).unwrap();
        let message = bot.send_message(&chat_id,
                                       "<b>test</b>",
                                       Some(&ParseMode::Html),
                                       None,
                                       None,
                                       None,
                                       None);
        assert!(message.is_ok());
    }

    #[test]
    fn test_url_chat_actions() {
        let (token, chat_id) = setup();
        let bot_url = [BASE_URL, &token].concat();
        let bot = Bot::new(bot_url).unwrap();

        let success = bot.send_chat_action(&chat_id, &ChatAction::FindLocation);
        assert!(success.is_ok());
    }

    #[test]
    fn test_url_inline_keyboard() {
        let (token, chat_id) = setup();
        let bot_url = [BASE_URL, &token].concat();
        let bot = Bot::new(bot_url).unwrap();

        let mut buttons = Vec::<Vec<InlineKeyboardButton>>::new();
        let mut row = Vec::<InlineKeyboardButton>::new();
        row.push(InlineKeyboardButton::new("TestButton".to_string(),
                                           Some("http://github.com/voider1/teleborg".to_string()),
                                           None,
                                           None,
                                           None));
        buttons.push(row);
        let markup = InlineKeyboardMarkup::new(buttons);
        let message = bot.send_message(&chat_id, "test", None, None, None, None, Some(&markup));
        assert!(message.is_ok());
    }

    #[test]
    fn test_send_contact() {
        let (token, chat_id) = setup();
        let bot_url = [BASE_URL, &token].concat();
        let bot = Bot::new(bot_url).unwrap();

        let contact = Contact {
            phone_number: "0612345678".to_string(),
            first_name: "Voider".to_string(),
            last_name: Some("1".to_string()),
            user_id: Some(666),
        };
        let message = bot.send_contact(&chat_id, &contact, None, None, None);
        assert!(message.is_ok());
    }
}
