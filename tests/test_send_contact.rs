extern crate teleborg;
extern crate reqwest;

#[cfg(test)]
mod tests {
    use teleborg::{Bot, ParseMode, ChatAction};
    use teleborg::objects::{Button, Markup, Contact};
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
    fn test_send_contact() {
        let (token, chat_id) = setup();
        let bot_url = [BASE_URL, &token].concat();
        let bot = Bot::new(bot_url).unwrap();

        let contact = Contact::new("0612345678".to_string(), "Voider".to_string(), Some("1".to_string()), Some(666));
        let message = bot.send_contact(&chat_id, &contact, None, None, None);
        println!("{:?}", &message);
        assert!(message.is_ok());
    }
}