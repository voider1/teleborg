use std::io::Read;
use std::thread;
use std::sync::mpsc;

use reqwest::Client;
use serde_json;
use serde_json::value::Map;

use error::Result;
use error::Error::{RequestFailed, JsonNotFound};

/// A struct which contains things associated with the bot.
#[derive(Debug)]
pub struct Bot {
    id: i64,
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
        let rjson = Bot::get_me(&client, &bot_url)?;
        let id = rjson.find("id").ok_or(JsonNotFound)?.as_i64().unwrap();
        let first_name = ::string_from_json(&rjson, "first_name").ok_or(JsonNotFound)?;
        let last_name = ::string_from_json(&rjson, "last_name");
        let username = ::string_from_json(&rjson, "username");

        Ok(Bot {
            id: id,
            first_name: first_name,
            last_name: last_name,
            username: username,
            client: client,
            bot_url: bot_url,
        })
    }

    pub fn get_updates(&self, rx: mpsc::Receiver<String>) {
        thread::spawn(|| {
            loop {
                unimplemented!();
            }
        });
    }

    /// Gets the information about the bot.
    fn get_me(client: &Client, bot_url: &str) -> Result<serde_json::Value> {
        let path = ["getMe"];
        let url = ::construct_api_url(bot_url, &path);
        let mut resp = client.get(&url).send()?;

        if resp.status().is_success() {
            let rjson: serde_json::Value = resp.json()?;
	        rjson.find("result").cloned().ok_or(JsonNotFound)
	    } else {
	        Err(RequestFailed(*resp.status()))
        }
	}
}
