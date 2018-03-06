use failure::ResultExt;
use reqwest::Client;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json;
use serde_json::Value;

use error::{ErrorKind, Result};
use types::{Update, User};

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
    pub bot_url: String,
    client: Client,
}

#[derive(Debug, Deserialize)]
struct TelegramResponse {
    ok: bool,
    error_code: Option<i32>,
    description: Option<String>,
    result: Option<Value>,
}

impl Bot {
    /// Constructs a new `Bot`.
    pub fn new(bot_url: String) -> Result<Self> {
        debug!("Going to construct a new Bot...");
        let client = Client::builder()
            .build()
            .context(ErrorKind::URLParsingError)?;
        let me = Self::get_me(&client, &bot_url)?;
        let id = me.id;
        let first_name = me.first_name;
        let last_name = me.last_name;
        let username = me.username.expect("Cannot find username of the bot");

        Ok(Bot {
            id,
            first_name,
            last_name,
            username,
            client,
            bot_url,
        })
    }

    fn get_me(client: &Client, bot_url: &str) -> Result<User> {
        debug!("Calling get_me...");
        let path = ["getMe"];
        let url = ::construct_api_url(bot_url, &path);
        let resp = client
            .get(&url)
            .send()
            .context(ErrorKind::NetworkingError)?
            .json()
            .context(ErrorKind::JSONDeserializationError)?;
        let user: User = Self::get_result(resp)?;
        Ok(user)
    }

    /// API call which will get called to get the updates for your bot.
    pub fn get_updates(
        &self,
        offset: i32,
        limit: Option<i32>,
        timeout: i32,
        network_delay: f32,
    ) -> Result<Option<Vec<Update>>> {
        debug!("Calling get_updates...");
        let limit = limit.unwrap_or(100);
        let path = ["getUpdates"];
        let path_url = ::construct_api_url(&self.bot_url, &path);
        let url = format!(
            "{}?offset={}&limit={}&timeout={}&network_delay={}",
            path_url, offset, limit, timeout, network_delay
        );
        let resp = self.client
            .get(&url)
            .send()
            .context(ErrorKind::NetworkingError)?
            .json()
            .context(ErrorKind::JSONDeserializationError)?;
        let updates: Vec<Update> = Self::get_result(resp)?;

        if updates.is_empty() {
            Ok(None)
        } else {
            Ok(Some(updates))
        }
    }

    /// The actual networking done for making API calls.
    pub fn call<T, R>(&self, path: &str, request: &T) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        debug!("Making API call...");
        let url = [&self.bot_url, path].join("/");
        let resp = self.client
            .post(&url)
            .json(&request)
            .send()
            .context(ErrorKind::NetworkingError)?
            .json()
            .context(ErrorKind::JSONDeserializationError)?;
        let result = Self::get_result(resp)?;

        Ok(result)
    }

    fn get_result<R>(resp: TelegramResponse) -> Result<R>
    where
        R: DeserializeOwned,
    {
        if resp.ok {
            let result_val = resp.result.ok_or(ErrorKind::JSONDeserializationError)?;
            let result: R =
                serde_json::from_value(result_val).context(ErrorKind::JSONDeserializationError)?;
            Ok(result)
        } else {
            Err(ErrorKind::TelegramAPIError)?
        }
    }
}
