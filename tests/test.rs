extern crate teleborg;

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Read;

    use teleborg::updater::*;

    const BOT_TOKEN: &'static str = "";

    #[test]
    fn create_bot() {
        let token = Some(BOT_TOKEN.to_string());
        let updater = Updater::new(token);
        println!("{}", updater.get_updates());
    }
}
