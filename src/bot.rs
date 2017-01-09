use std::io::Read;
use std::thread;

use reqwest::Client;
use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use error::Result;
use error::Error::{RequestFailed, JsonNotFound};
use objects::update::Update;
use objects::user::User;
use objects::message::Message;
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
    pub fn new(bot_url: String) -> Result<Self> {
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

    pub fn send_message(&self,
                        chat_id: &i64,
                        text: &str,
                        parse_mode: Option<&str>,
                        disable_web_page_preview: Option<&bool>,
                        disable_notification: Option<&bool>,
                        reply_to_message_id: Option<&i32>)
                        -> Result<Message> {
        let chat_id: &str = &chat_id.to_string();
        let parse_mode = parse_mode.unwrap_or("None");
        let disable_web_page_preview: &str = &disable_web_page_preview.unwrap_or(&false)
            .to_string();
        let disable_notification: &str = &disable_notification.unwrap_or(&false).to_string();
        let reply_to_message_id: &str = &reply_to_message_id.map(|i| i.to_string())
            .unwrap_or("None".to_string());
        let path = ["sendMessage"];
        let url = ::construct_api_url(&self.bot_url, &path);
        let params = [("chat_id", chat_id),
                      ("text", text),
                      ("parse_mode", parse_mode),
                      ("disable_web_page_preview", disable_web_page_preview),
                      ("disable_notification", disable_notification),
                      ("reply_to_message_id", reply_to_message_id)];
        let mut data = self.client.post(&url).form(&params).send()?;
        let rjson: Value = data.json()?;
        let message_json = rjson.find("result").ok_or(JsonNotFound)?;
        let message: Message = serde_json::from_value(message_json.clone())?;
        Ok(message)
    }

    pub fn reply_to_message(&self, update: &Update, text: &str) -> Result<Message> {
        let chat_id = update.clone().message.unwrap().chat.id;
        self.send_message(&chat_id, text, None, None, None, None)
    }
}
