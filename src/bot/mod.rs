pub use self::parse_mode::ParseMode;
pub use self::chat_action::ChatAction;

mod parse_mode;
mod chat_action;
mod messaging;

use reqwest::Client;
use serde_json;
use serde_json::Value;

use error::{Result, check_for_error};
use error::Error::{RequestFailed, JsonNotFound};
use objects::Update;
use value_extension::ValueExtension;

/// A `Bot` which will do all the API calls.
///
/// The `Bot` will be given access to in a `Command` with which you can do all
/// the API interactions in your `Command`s.
#[derive(Debug)]
pub struct Bot {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: String,
    client: Client,
    pub bot_url: String,
}

impl Bot {
    /// Constructs a new `Bot`.
    pub fn new(bot_url: String) -> Result<Self> {
        let client = Client::new()?;
        let rjson = Bot::get_me(&client, &bot_url)?;
        let id = rjson.as_required_i64("id")?;
        let first_name = rjson.as_required_string("first_name")?;
        let last_name = rjson.as_optional_string("last_name");
        let username = rjson.as_required_string("username")?;

        Ok(Bot {
            id: id,
            first_name: first_name,
            last_name: last_name,
            username: username,
            client: client,
            bot_url: bot_url,
        })
    }

    /// API call which gets the information about your bot.
    pub fn get_me(client: &Client, bot_url: &str) -> Result<Value> {
        let path = ["getMe"];
        let url = ::construct_api_url(bot_url, &path);
        let mut resp = client.get(&url).send()?;

        if resp.status().is_success() {
            let rjson: Value = resp.json()?;
            rjson.get("result").cloned().ok_or(JsonNotFound)
        } else {
            Err(RequestFailed(*resp.status()))
        }
    }

    /// API call which will get called to get the updates for your bot.
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
        let mut data = self.client.get(&url).send()?;
        let rjson: Value = check_for_error(data.json()?)?;
        let updates_json = rjson.get("result");

        if let Some(result) = updates_json {
            let updates: Vec<Update> = serde_json::from_value(result.clone())?;
            Ok(Some(updates))
        } else {
            Ok(None)
        }
    }
}
