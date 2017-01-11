extern crate teleborg;

#[cfg(test)]
mod tests {
    use super::*;
    use teleborg::command_handler;
    use teleborg::bot::{Bot, ParseMode};
    use teleborg::objects::update::Update;
    use teleborg::updater;

    #[test]
    fn create_updater() {
        let mut commands = command_handler::CommandHandler::new();
        commands.add("test", test);
        updater::Updater::start(None, None, None, None, commands);
    }

    fn test(bot: &Bot, update: Update) {
        let chat_id = &update.message.unwrap().chat.id;
        let x = bot.send_message(chat_id,
                                 "<p>Test</p>",
                                 Some(&ParseMode::Html),
                                 None,
                                 None,
                                 None);
        println!("{:?}", x);
    }
}
