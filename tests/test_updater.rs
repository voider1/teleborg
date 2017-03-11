extern crate teleborg;

#[cfg(test)]
mod tests {
    use teleborg::dispatcher;
    use teleborg::updater::Updater;
    use teleborg::bot::Bot;
    use teleborg::objects::update::Update;

    fn test_updater() {
        let mut dispatcher = dispatcher::Dispatcher::new();
        Updater::start(None, None, None, None, dispatcher);
    }
}
