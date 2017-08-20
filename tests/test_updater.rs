extern crate teleborg;

#[cfg(test)]
mod tests {
    use teleborg::{Dispatcher, Updater, Bot};
    use teleborg::objects::Update;

    #[test]
    fn test_updater() {
        let mut dispatcher = Dispatcher::new();
        dispatcher.add_command_handler("test", test, false);
        dispatcher.add_inline_query_handler(test_inline_query);
        Updater::start(None, None, None, None, dispatcher);
    }

    fn test(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        bot.reply_to_message(&update, "It works!").unwrap();
    }

    fn test_inline_query(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        if let Some(inline_query) = update.inline_query {
            println!("query: {}", inline_query.query);
        }
    }
}
