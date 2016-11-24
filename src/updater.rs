use bot;

const BASE_URL: &'static str = "https://api.telegram.org/bot";

pub struct Updater {
	token: String,
	bot: bot::Bot,
}

impl Updater {
    pub fn new(token: String) -> Updater {
		let bot_url = [BASE_URL, &token].concat();
		let bot = bot::Bot::new(bot_url);

		Updater {
			token: token,
			bot: bot,
		}
    }
}
