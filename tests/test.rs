extern crate teleborg;

#[cfg(test)]
mod tests {
    use super::*;

    use teleborg::updater::Updater;
    use teleborg::bot;
    use teleborg::objects::update;
    use teleborg::command_handler::CommandHandler;

    #[test]
    fn create_updater() {
        let mut command_handler = CommandHandler::new();
        command_handler.add("foo", foo);
        Updater::start(None, None, None, None, command_handler);
    }

    fn foo(bot: &bot::Bot, update: &update::Update) {
        println!("IT WORKS OMG");
    }
}
