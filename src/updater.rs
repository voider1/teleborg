use std::env;
use std::thread;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::io::Read;

use reqwest;
use json;

use bot;
use update::Update;

const BASE_URL: &'static str = "https://api.telegram.org/bot";

pub struct Updater {
	token: String,
	bot: bot::Bot,
	client: reqwest::Client,
}

impl Updater {
	/// Creates a new Updater struct.
    pub fn new(token: Option<String>) -> Result<Arc<Updater>, reqwest::Error> {
        let token = token.or_else(|| env::var("TELEGRAM_BOT_TOKEN").ok()).
                expect("You should pass in a token to new or set TELEGRAM_BOT_TOKEN");

        let bot_url = [BASE_URL, &token].concat();
		let bot = bot::Bot::new(bot_url)?;
		let client = reqwest::Client::new()?;

		Ok(Arc::new(Updater {
			token: token,
			bot: bot,
            client: client,
		}))
    }

    pub fn start_polling(this: Arc<Self>) {
        let (tx, rx) = channel();
        let update = this.get_updates()?;

        thread::spawn(move|| {
            loop {
                tx.send(this.get_updates()).unwrap();
            }
        });
    }

    /// Function to get all the messages for the bot.
    pub fn get_updates<E>(&self) -> Result<Update, E> {
    	let path = ["getUpdates"];
    	let url = ::construct_api_url(&self.bot.bot_url, &path);
    	let params = [("timeout", 30)];
    	let mut resp = self.client.get(&url).form(&params).send()?;
		let mut body = String::new();
		resp.read_to_string(&mut body)?;
		let rjson = json::parse(&body)?;

		Ok(Update::new(rjson["result"].clone()))
    }
}
