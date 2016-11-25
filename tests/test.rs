extern crate teleborg;

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Read;

    use teleborg::updater::*;

    const BOT_ID: &'static str = "";

    #[test]
    fn create_bot() {
        let updater = Updater::new(BOT_ID.to_string());
        let mut resp = updater.get_updates();
    }
}
