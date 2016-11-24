use reqwest;

use bot;

const BASE_URL: &'static str = "https://api.telegram.org/bot";

pub struct Updater {
	token: String,
	bot: bot::Bot,
	client: reqwest::Client,
}

impl Updater {
    pub fn new(token: String) -> Updater {
		let bot_url = [BASE_URL, &token].concat();
		let bot = bot::Bot::new(bot_url);
		let client = reqwest::Client::new().unwrap();

		Updater {
			token: token,
			bot: bot,
			client: client,
		}
    }

    /// Function to get all the messages for the bot.
    pub fn get_updates(&self) -> reqwest::Response {
    	let path = ["getUpdates"];
    	let url = ::construct_api_url(&self.bot.bot_url, &path);
    	self.client.get(&url).send().unwrap()	
    }
}
