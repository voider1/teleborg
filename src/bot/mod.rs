pub use self::parse_mode::ParseMode;
pub use self::chat_action::ChatAction;

mod parse_mode;
mod chat_action;

use reqwest::Client;
use serde_json;
use serde_json::Value;

use bot::chat_action::{get_chat_action};
use bot::parse_mode::{get_parse_mode};
use error::{Result, check_for_error};
use error::Error::{RequestFailed, JsonNotFound};
use objects::{Update, Message, Contact, InlineKeyboardMarkup};
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

    /// API call which will send a message to a chat which your bot participates in.
    pub fn send_message(&self,
                        chat_id: &i64,
                        text: &str,
                        parse_mode: Option<&ParseMode>,
                        disable_web_page_preview: Option<&bool>,
                        disable_notification: Option<&bool>,
                        reply_to_message_id: Option<&i64>,
                        reply_markup: Option<&InlineKeyboardMarkup>)
                        -> Result<Message> {
        let chat_id: &str = &chat_id.to_string();
        let parse_mode = &get_parse_mode(parse_mode.unwrap_or(&ParseMode::Text));
        let disable_web_page_preview: &str = &disable_web_page_preview.unwrap_or(&false)
            .to_string();
        let disable_notification: &str = &disable_notification.unwrap_or(&false).to_string();
        let reply_to_message_id: &str = &reply_to_message_id.map(|i| i.to_string())
            .unwrap_or("None".to_string());
        let reply_markup = &reply_markup.map(|r| serde_json::to_string(r).unwrap_or("".to_string()));
        let path = ["sendMessage"];
        let params = [("chat_id", chat_id),
                      ("text", text),
                      ("parse_mode", parse_mode),
                      ("disable_web_page_preview", disable_web_page_preview),
                      ("disable_notification", disable_notification),
                      ("reply_to_message_id", reply_to_message_id),
                      ("reply_markup", &reply_markup.unwrap())];
        self.post_message(&path, &params)
    }

    /// API call which will reply to a message directed to your bot.
    pub fn reply_to_message(&self, update: &Update, text: &str) -> Result<Message> {
        let message = update.clone().message.unwrap();
        let message_id = message.message_id;
        let chat_id = message.chat.id;
        self.send_message(&chat_id, text, None, None, None, Some(&message_id), None)
    }

    /// API call which will forward a message.
    pub fn forward_messge(&self,
                          update: &Update,
                          chat_id: &i64,
                          disable_notification: Option<&bool>)
                          -> Result<Message> {
        let message = update.clone().message.unwrap();
        let chat_id: &str = &chat_id.to_string();
        let from_chat_id: &str = &message.chat.id.to_string();
        let message_id: &str = &message.message_id.to_string();
        let disable_notification: &str = &disable_notification.unwrap_or(&false).to_string();
        let path = ["forwardMessage"];
        let params = [("chat_id", chat_id),
                      ("from_chat_id", from_chat_id),
                      ("disable_notification", disable_notification),
                      ("message_id", message_id)];
        self.post_message(&path, &params)
    }

    /// API call which will send the requested chat action to your bot.
    pub fn send_chat_action(&self, chat_id: &i64, action: &ChatAction) -> Result<Message> {
        let action = get_chat_action(action);
        let path = ["sendChatAction"];
        let params = [("chat_id", &chat_id.to_string()),
                      ("action", &action)];
        self.post_message(&path, &params)
    }

    /// API call which will send the given contact.
    pub fn send_contact(&self,
                        chat_id: &i64,
                        contact: &Contact,
                        disable_notification: Option<&bool>,
                        reply_to_message_id: Option<&i32>,
                        reply_markup: Option<&InlineKeyboardMarkup>) -> Result<Message> {

        let chat_id: &str = &chat_id.to_string();
        let disable_notification: &str = &disable_notification.unwrap_or(&false).to_string();
        let reply_to_message_id: &str = &reply_to_message_id.map(|i| i.to_string())
            .unwrap_or("None".to_string());
        let reply_markup = &reply_markup.map(|r| serde_json::to_string(r).unwrap_or("".to_string()));

        let path = ["sendContact"];
        let url = ::construct_api_url(&self.bot_url, &path);
        let params = [("chat_id", chat_id),
                      ("phone_number", &contact.phone_number),
                      ("first_name", &contact.first_name),
                      ("last_name", &contact.clone().last_name.unwrap()),
                      ("disable_notification", disable_notification),
                      ("reply_to_message_id", reply_to_message_id),
                      ("reply_markup", &reply_markup.unwrap())];
        self.post_message(&path, &params)
    }

    /// The actual networking done for sending messages.
    fn post_message(&self, path: &[&str], params: &[(&str, &str)]) -> Result<Message> {
        let url = ::construct_api_url(&self.bot_url, path);
        let mut data = self.client.post(&url).form(&params).send()?;
        let rjson: Value = check_for_error(data.json()?)?;
        let message_json = rjson.get("result").ok_or(JsonNotFound)?;
        let message: Message = serde_json::from_value(message_json.clone())?;
        Ok(message)
    }
}
