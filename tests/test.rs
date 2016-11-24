extern crate teleborg;

use std::io::Read;

#[cfg(test)]
mod tests {
    use teleborg::updater::*;
    const BOT_ID: &'static str = "";

    #[test]
    fn create_bot() {
        let updater = Updater::new(BOT_ID.to_string());
        let resp = updater.get_updates();
        if resp.status().is_success() {
            let mut body = String::new();
            resp.read_to_string(&mut body).unwrap();
            println!("{}", &body);
        } else {
            panic!("ERROR!");
        }
    }
}
