use std::io::Read;
use std::sync::mpsc;

use reqwest::Client;
use serde_json;
use serde_json::value::Map;

use error::Result;
use error::Error::RequestFailed;

/// A struct which contains things associated with the bot.
#[derive(Debug)]
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
    pub fn new(bot_url: String) -> Result<Bot> {
        let client = Client::new()?;
        let mut rjson = Bot::get_me(&client, &bot_url)?;

        Ok(Bot {
            id: rjson.get("id"),
            first_name: rjson["first_name"].take_string().unwrap(),
            last_name: rjson["last_name"].take_string(),
            username: rjson["username"].take_string(),
            client: client,
            bot_url: bot_url,
        })
    }

    pub fn handle_updates(&self, rx: mpsc::Receiver<String>) {
        !unimplemented!()
    }

    /// Gets the information about the bot.
    fn get_me(client: &Client, bot_url: &str) -> Result<Map> {
        let path = ["getMe"];
        let url = ::construct_api_url(bot_url, &path);
        let mut resp = client.get(&url).send()?;

        if resp.status().is_success() {
            let data: serde_json::Value = resp.json()?;
            let rjson = data.as_object().unwrap();
	        Ok(*rjson.get("result"))
	    } else {
	        Err(RequestFailed(*resp.status()))
        }
	}
}
