extern crate reqwest;

use reqwest::{Response};

pub const BASE_URL: &'static str = "https://api.telegram.org/bot";

/// A struct which contains things associated with the bot.
pub struct Bot {
	token: String,
	bot_url: String,
}

impl Bot {
    /// Return a new bot struct.
    /// Panic! if something goes wrong.
	pub fn new(token: String) -> Bot {

		let bot_url = [BASE_URL, &token].concat();
		let bot = Bot {
			token: token,
			bot_url: bot_url,
		};
        let response = bot.get_me();

        if response.status().is_success() {
            bot
        } else {
            panic!("An error has occured.");
        }
	}

    /// A simple test to see if the bot exists.
	fn get_me(&self) -> Response {
	    let path = ["getMe"];
	    let url = construct_api_url(self, &path);
	    reqwest::get(&url).unwrap()
    }
}

/// Construct an API URL with the base bot URL and an
/// array of strings which will construct the path.
fn construct_api_url(bot: &Bot, path: &[&str]) -> String {

    format!("{}/{}", bot.bot_url, path.join("/"))
}
