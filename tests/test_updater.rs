extern crate teleborg;

#[cfg(test)]
mod tests {
    use teleborg::dispatcher;
    use teleborg::updater::Updater;
    use teleborg::bot::Bot;
    use teleborg::objects::update::Update;

    fn test_updater() {
        let mut dispatcher = dispatcher::Dispatcher::new();
        dispatcher.add_command_handler("test", test, false);
        Updater::start(None, None, None, None, dispatcher);
    }

    fn test(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        println!("{:?}", args);
    }
}
