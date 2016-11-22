extern crate teleborg;

#[cfg(test)]
mod tests {
    use teleborg::*;
    const BOT_ID: &'static str = "281675190:AAGG60c26-fBcfQJxU_1lTqoZyWMVPVl0EQ";

    #[test]
    fn create_bot() {
        Bot::new(BOT_ID.to_string());
    }
}
