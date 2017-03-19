extern crate teleborg;

#[cfg(test)]
mod tests {
    use teleborg::{Dispatcher, Updater, Bot};
    use teleborg::objects::Update;

    fn test_updater() {
        let mut dispatcher = Dispatcher::new();
        dispatcher.add_command_handler("test", test, false);
        Updater::start(None, None, None, None, dispatcher);
    }

    fn test(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        bot.reply_to_message(&update, "It works!").unwrap();
    }
}
