extern crate teleborg;

#[cfg(test)]
mod tests {
    use super::*;

    use teleborg::updater::Updater;
    use teleborg::bot;
    use teleborg::objects::update;
    use teleborg::command_handler::CommandHandler;
    use teleborg::command::Command;

    #[test]
    fn create_updater() {
        let mut command_handler = CommandHandler::new();
        command_handler.add("foo", foo);
        command_handler.add("hitler", hitler);
        Updater::start(None, None, None, None, command_handler);
    }

    fn foo(bot: &bot::Bot, update: update::Update) {
        let chat_id = &update.message.unwrap().chat.id;
        bot.send_message(chat_id, "Test!!!", None, None, None, None).unwrap();
        println!("IT WORKS OMG");
    }

    fn hitler(bot: &bot::Bot, update: update::Update) {
        bot.reply_to_message(&update, "SIEG HEIL").unwrap();
    }
}
