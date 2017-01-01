use std::env;
use std::thread;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::io::Read;

use reqwest;
use serde_json;

use bot;
use update::Update;
use error::Result;

const BASE_URL: &'static str = "https://api.telegram.org/bot";

#[derive(Debug)]
pub struct Updater {
	token: String,
	bot: bot::Bot,
	client: reqwest::Client,
}

impl Updater {
	/// Creates a new Updater struct.
    pub fn new(token: Option<String>) -> Result<Updater> {
        let token = token.or_else(|| env::var("TELEGRAM_BOT_TOKEN").ok())
                .expect("You should pass in a token to new or set TELEGRAM_BOT_TOKEN");

        let bot_url = [BASE_URL, &token].concat();
		let bot = bot::Bot::new(bot_url)?;
		let client = reqwest::Client::new()?;

		Ok(Updater {
			token: token,
			bot: bot,
            client: client,
		})
    }

    pub fn start_polling(&self) {
        unimplemented!();
    }

    fn start_polling_thread(&self) {
        loop {
            unimplemented!();
        }
    }
}
