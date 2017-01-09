extern crate teleborg;

#[cfg(test)]
mod tests {
    use super::*;

    use teleborg::updater::Updater;
    use teleborg::bot;
    use teleborg::objects::update;
    use teleborg::command_handler::CommandHandler;
    use teleborg::command::Command;

    struct Test;

    impl Command for Test {
        fn execute(&mut self, bot: &bot::Bot, update: &update::Update) {
            println!("Hello");
        }
    }

    #[test]
    fn create_updater() {
        let mut command_handler = CommandHandler::new();
        command_handler.add("foo", foo);
        command_handler.add("test", Test);
        Updater::start(None, None, None, None, command_handler);
    }

    fn foo(bot: &bot::Bot, update: &update::Update) {
        println!("IT WORKS OMG");
    }
}
