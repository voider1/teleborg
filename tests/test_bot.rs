extern crate teleborg;
extern crate reqwest;

#[cfg(test)]
mod tests {
    use std::env;

    use reqwest::Client;

    use teleborg::{Bot, ParseMode, ChatAction, NO_MARKUP};
    use teleborg::objects::{Contact, InlineKeyboardButton, InlineKeyboardMarkup, ForceReply};

    const BASE_URL: &'static str = "https://api.telegram.org/bot";

    fn get_token() -> String {
        env::var("TELEGRAM_BOT_TOKEN").ok().expect(
            "Can't find TELEGRAM_BOT_TOKEN env variable",
        )
    }

    fn setup() -> (Bot, i64, i64) {
        let token = get_token();
        let bot_url = [BASE_URL, &token].concat();
        let bot = Bot::new(bot_url).unwrap();
        let chat_id = env::var("TELEGRAM_CHAT_ID")
            .ok()
            .expect("Can't find TELEGRAM_CHAT_ID env variable")
            .parse::<i64>()
            .unwrap();
        let user_id = env::var("TELEGRAM_USER_ID")
            .ok()
            .expect("Can't find TELEGRAM_USER_ID env variable")
            .parse::<i64>()
            .unwrap();
        (bot, chat_id, user_id)
    }

    #[test]
    fn test_get_me() {
        let token = get_token();
        let bot_url = [BASE_URL, &token].concat();
        let client = Client::new();
        let bot = Bot::get_me(&client, &bot_url);
        assert!(bot.is_ok());
    }

    #[test]
    fn test_send_message() {
        let (bot, chat_id, _) = setup();
        let message = bot.send_message(
            &chat_id,
            "test send_message",
            None,
            None,
            None,
            None,
            NO_MARKUP,
        );
        assert!(message.is_ok());
    }

    #[test]
    fn test_send_text_message() {
        let (bot, chat_id, _) = setup();
        let message = bot.send_message(
            &chat_id,
            "test send text message",
            Some(&ParseMode::Text),
            None,
            None,
            None,
            NO_MARKUP,
        );
        assert!(message.is_ok());
    }

    #[test]
    fn test_send_markdown_message() {
        let (bot, chat_id, _) = setup();
        let message = bot.send_message(
            &chat_id,
            "*test send bold message*",
            Some(&ParseMode::Markdown),
            None,
            None,
            None,
            NO_MARKUP,
        );
        assert!(message.is_ok());
    }

    #[test]
    fn test_send_html_message() {
        let (bot, chat_id, _) = setup();
        let message = bot.send_message(
            &chat_id,
            "<b>test send HTML message</b>",
            Some(&ParseMode::Html),
            None,
            None,
            None,
            NO_MARKUP,
        );
        assert!(message.is_ok());
    }

    #[test]
    fn test_url_chat_actions() {
        let (bot, chat_id, _) = setup();
        let success = bot.send_chat_action(&chat_id, &ChatAction::FindLocation);
        assert!(success.is_ok());
    }

    #[test]
    fn test_url_inline_keyboard() {
        let (bot, chat_id, _) = setup();
        let mut buttons = Vec::<Vec<InlineKeyboardButton>>::new();
        let mut row = Vec::<InlineKeyboardButton>::new();
        row.push(InlineKeyboardButton::new(
            "TestButton".to_string(),
            Some("http://github.com/voider1/teleborg".to_string()),
            None,
            None,
            None,
        ));
        buttons.push(row);
        let markup = InlineKeyboardMarkup::new(buttons);
        let message = bot.send_message(
            &chat_id,
            "test inline keyboard",
            None,
            None,
            None,
            None,
            Some(markup),
        );
        assert!(message.is_ok());
    }

    #[test]
    fn test_force_reply() {
        let (bot, chat_id, _) = setup();
        let force_reply = ForceReply::new(true, None);
        let message = bot.send_message(
            &chat_id,
            "test force reply",
            None,
            None,
            None,
            None,
            Some(force_reply),
        );
        assert!(message.is_ok());
    }

    #[test]
    fn test_send_contact() {
        let (bot, chat_id, _) = setup();

        let contact = Contact {
            phone_number: "0612345678".to_string(),
            first_name: "Voider".to_string(),
            last_name: Some("1".to_string()),
            user_id: Some(666),
        };
        let message = bot.send_contact(&chat_id, &contact, None, None, NO_MARKUP);
        assert!(message.is_ok());
    }

    #[test]
    fn test_get_user_profile_photos() {
        let (bot, _, user_id) = setup();
        let user_profile_photos = bot.get_user_profile_photos(&user_id, None, None);
        assert!(user_profile_photos.is_ok());
    }

    #[test]
    fn test_get_file() {
        let (bot, _, _) = setup();
        let file_id = "AgADBAADXqsxG-2w-FNEGNmL3fISo9pw4BkABGDC7Yru_9tn5bAAAgI";
        let file = bot.get_file(file_id);
        assert!(file.is_ok());
    }
}
