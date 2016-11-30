use std::result::Result;
use std::io::Read;
use std::sync::mpsc;

use reqwest::Client;
use json;

use errors::boterror::BotError;

/// A struct which contains things associated with the bot.
pub struct Bot {
    id: i32,
    first_name: String,
    last_name: Option<String>,
    username: Option<String>,
    client: Client,
    pub bot_url: String,
}

impl Bot {
    /// Return a new bot struct.
    /// Panic! if something goes wrong.
    pub fn new(bot_url: String) -> Result<Bot, BotError> {
        let client = Client::new()?;
        let mut rjson = Bot::get_me(&client, &bot_url)?;

        Ok(Bot {
            id: rjson["id"].as_i32().unwrap(),
            first_name: rjson["first_name"].take_string().unwrap(),
            last_name: rjson["last_name"].take_string(),
            username: rjson["username"].take_string(),
            client: client,
            bot_url: bot_url,
        })
    }

    pub fn handle_updates(&self, rx: mpsc::Receiver<String>) {
        loop {
            if let Ok(val) = rx.try_recv() {
                let rjson = json::parse(&val).unwrap();
                println!("{}", rjson["resut"]);
            } else {
                continue;
            }
        }
    }

    /// Gets the information about the bot.
    fn get_me(client: &Client, bot_url: &str) -> Result<json::JsonValue, BotError> {
        let path = ["getMe"];
        let url = ::construct_api_url(bot_url, &path);
        let mut resp = client.get(&url).send()?;

        if resp.status().is_success() {
	        let mut body = String::new();
	        resp.read_to_string(&mut body)?;
	        let rjson = json::parse(&body)?;
	        Ok(rjson["result"].clone())
	    } else {
	    	Err(BotError::BadRequest("The request was unsuccessful".to_string()))
	    }
    }
}
