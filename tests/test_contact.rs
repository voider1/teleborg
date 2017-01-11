extern crate teleborg;

#[cfg(test)]
mod tests {
    use teleborg::bot::Bot;
    use teleborg::objects::update::Update;
    use teleborg::updater;
    
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

    // This test tries to send a contact by sending a
    // simple phone number (+1 123-456-7890) along with
    // "Test Contact" as the first/last name for the contact.
    #[test]
    fn test_contact() {
        let (token, chat_id) = setup();
        let bot_url = [BASE_URL, &token].concat();
        let bot = Bot::new(bot_url).unwrap();
        let contact = bot.send_contact(&chat_id,
                                 "+1 123-456-7890",
                                 "Test",
                                 "Contact",
                                 None,
                                 None);
        println!("{:?}", contact);
        assert!(contact.is_ok());
    }
}
