extern crate teleborg;

#[cfg(test)]
mod tests {
    use teleborg::command_handler;
    use teleborg::updater::Updater;

    #[test]
    fn test_updater() {
        let mut commands = command_handler::CommandHandler::new();
        Updater::start(None, None, None, None, commands);
        Updater::stop();
    }
}
