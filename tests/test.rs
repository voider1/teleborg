extern crate teleborg;

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Read;

    use teleborg::updater::*;

    #[test]
    fn create_bot() {
        let updater = Updater::new(None);
        let rjson = Updater::start_polling(updater.clone());
        println!("{}", updater.get_updates());
    }
}
