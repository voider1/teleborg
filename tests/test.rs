extern crate teleborg;

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Read;
    use std::thread::sleep;
    use std::time::Duration;

    use teleborg::updater::*;

    #[test]
    fn create_updater() {
        let updater = Updater::new(None);
        updater.unwrap().start_polling(None, None, None);
        println!("asd");
        sleep(Duration::from_secs(10));
        assert!(true);
    }
}
