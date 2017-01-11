extern crate teleborg;

#[cfg(test)]
mod tests {
    use super::*;
    use teleborg::command_handler;
    use teleborg::bot::{Bot, ParseMode};
    use teleborg::objects::update::Update;
    use teleborg::updater;
    use teleborg::objects::inline_keyboard::Markup;
    use teleborg::objects::inline_keyboard::Button;

    #[test]
    fn create_updater() {
        let mut commands = command_handler::CommandHandler::new();
        commands.add("test", test);
        updater::Updater::start(None, None, None, None, commands);
    }

    fn test(bot: &Bot, update: Update) {
        let chat_id = &update.message.unwrap().chat.id;
        let mut buttons = Vec::<Button>::new();
        buttons.push(Button::new("TestButton".to_string(), Some("http://github.com/voider1/teleborg".to_string())));
        let markup = Markup::new(buttons);
        let x = bot.send_message(chat_id,
                                 "GET THE NEW TELEBORG API!!!",
                                 Some(&ParseMode::Html),
                                 None,
                                 None,
                                 None, 
                                 Some(&markup));
        println!("{:?}", x);

    }
}
