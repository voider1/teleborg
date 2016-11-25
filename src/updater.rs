extern crate json;

use std::io::Read;

use reqwest;

use bot;

const BASE_URL: &'static str = "https://api.telegram.org/bot";

pub struct Updater {
	token: String,
	bot: bot::Bot,
	client: reqwest::Client,
}

impl Updater {
	/// Creates a new Updater struct.
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
    pub fn get_updates(&self) -> json::JsonValue {
    	let path = ["getUpdates"];
    	let url = ::construct_api_url(&self.bot.bot_url, &path);
    	let mut resp = self.client.get(&url).send().unwrap();
		let mut body = String::new();
		resp.read_to_string(&mut body).unwrap();

    	if resp.status().is_success() {
    		let rjson = json::parse(&body).unwrap();
    		rjson["result"].clone()
    	} else {
    		panic!(body);
    	}
    }
}
