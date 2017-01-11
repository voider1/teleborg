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
        commands.add("inl_test", test_inline_keyboard);
        updater::Updater::start(None, None, None, None, commands);
    }

    fn test(bot: &Bot, update: Update) {
        let x = bot.send_message(chat_id,
                                 "<p>paragraph</p>",
                                 Some(&ParseMode::Html),
                                 None,
                                 None,
                                 None, 
                                 None);
        println!("{:?}", x);

    }

    fn test_inline_keyboard(bot: &Bot, update: Update) {
        let chat_id = &update.message.unwrap().chat.id;
        
        let mut row = Vec::<Button>::new();
        row.push(Button::new("TestButton".to_string(), None, None, None, None));
        let mut total = Vec::<Vec<Button>>::new();
        total.push(row);
        let markup = Markup::new(total);

        let x = bot.send_message(chat_id,
                                 "GET THE NEW TELEBORG API!!!",
                                 None,
                                 None,
                                 None,
                                 None, 
                                 Some(&markup));
        println!("{:?}", x);
    }
}
