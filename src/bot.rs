use std::io::Read;
use std::thread;
use std::sync::mpsc;

use reqwest::Client;
use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use error::Result;
use error::Error::{RequestFailed, JsonNotFound};
use update::Update;
use objects::user::User;
use value_extension::ValueExtension;

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
    pub fn new(bot_url: String) -> Result<Bot> {
        let client = Client::new()?;
        let rjson = Bot::get_me(&client, &bot_url)?;
        let id = rjson.as_required_i64("id")?;
        let first_name = rjson.as_required_string("first_name")?;
        let last_name = rjson.as_optional_string("last_name");
        let username = rjson.as_optional_string("username");

        Ok(Bot {
            id: id,
            first_name: first_name,
            last_name: last_name,
            username: username,
            client: client,
            bot_url: bot_url,
        })
    }

    /// Gets the information about the bot.
    fn get_me(client: &Client, bot_url: &str) -> Result<Value> {
        let path = ["getMe"];
        let url = ::construct_api_url(bot_url, &path);
        let mut resp = client.get(&url).send()?;

        if resp.status().is_success() {
            let rjson: Value = resp.json()?;
            rjson.find("result").cloned().ok_or(JsonNotFound)
        } else {
            Err(RequestFailed(*resp.status()))
        }
    }

    pub fn get_updates(&self,
                       offset: i32,
                       limit: Option<i32>,
                       timeout: Option<i32>,
                       network_delay: Option<i32>)
                       -> Result<Option<Vec<Update>>> {
        let limit = limit.unwrap_or(100);
        let timeout = timeout.unwrap_or(0);
        // Use network_delay when it gets implemented
        let network_delay = network_delay.unwrap_or(5);
        let path = ["getUpdates"];
        let path_url = ::construct_api_url(&self.bot_url, &path);
        let url = format!("{}?offset={}&limit={}&timeout={}",
                          path_url,
                          offset,
                          limit,
                          timeout);
        let params = [("offset", offset), ("limit", limit), ("timeout", timeout)];
        let mut data = self.client.get(&url).send()?;
        let rjson: Value = data.json()?;
        let updates_json = rjson.find("result");

        if let Some(result) = updates_json {
            let updates: Vec<Update> = serde_json::from_value(result.clone())?;
            Ok(Some(updates))
        } else {
            Ok(None)
        }
    }
}
