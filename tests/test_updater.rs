extern crate teleborg;

#[cfg(test)]
mod tests {
    use teleborg::dispatcher;
    use teleborg::updater::Updater;

    #[test]
    fn test_updater() {
        let dispatcher = dispatcher::Dispatcher::new();
        let updater = Updater::start(None, None, None, None, dispatcher);
        updater.stop();
    }
}
