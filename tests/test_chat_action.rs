extern crate teleborg;
extern crate reqwest;

#[cfg(test)]
mod tests {
    use teleborg::{Bot, ParseMode, ChatAction};
    use teleborg::objects::{Button, Markup};
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
    fn test_url_chat_actions() {
        let (token, chat_id) = setup();
        let bot_url = [BASE_URL, &token].concat();
        let bot = Bot::new(bot_url).unwrap();

        let message = bot.send_chat_action(&chat_id, &ChatAction::FindLocation);
        println!("{:?}", &message);
        assert!(message.is_ok());
    }
}