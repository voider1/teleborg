pub use self::parse_mode::ParseMode;
pub use self::chat_action::ChatAction;

mod parse_mode;
mod chat_action;
mod file;

use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json;
use serde_json::Value;

use bot::chat_action::get_chat_action;
use bot::parse_mode::get_parse_mode;
use error::{Result, check_for_error};
use error::Error::JsonNotFound;
use marker::ReplyMarkup;
use objects::{Update, Message, Contact, InlineKeyboardMarkup, User, UserProfilePhotos, File};

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

impl Bot {
    /// Constructs a new `Bot`.
    pub fn new(bot_url: String) -> Result<Self> {
        debug!("Going to construct a new Bot...");
        let client = Client::new()?;
        let me = Bot::get_me(&client, &bot_url)?;
        let id = me.id;
        let first_name = me.first_name;
        let last_name = me.last_name;
        let username = me.username.expect("Cannot find username of the bot");

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
    pub fn get_me(client: &Client, bot_url: &str) -> Result<User> {
        debug!("Calling get_me...");
        let path = ["getMe"];
        let url = ::construct_api_url(bot_url, &path);
        let mut data = client.get(&url)?.send()?;
        let rjson: Value = check_for_error(data.json()?)?;
        let user_json = rjson.get("result").ok_or(JsonNotFound)?;
        let user: User = serde_json::from_value(user_json.clone())?;
        Ok(user)
    }

    /// API call which will get called to get the updates for your bot.
    pub fn get_updates(
        &self,
        offset: i32,
        limit: Option<i32>,
        timeout: Option<i32>,
        network_delay: Option<f32>,
    ) -> Result<Option<Vec<Update>>> {
        debug!("Calling get_updates...");
        let limit = limit.unwrap_or(100);
        let timeout = timeout.unwrap_or(0);
        let network_delay = network_delay.unwrap_or(0.0);
        let path = ["getUpdates"];
        let path_url = ::construct_api_url(&self.bot_url, &path);
        let url = format!(
            "{}?offset={}&limit={}&timeout={}&network_delay={}",
            path_url,
            offset,
            limit,
            timeout,
            network_delay
        );
        let mut data = self.client.get(&url)?.send()?;
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
    pub fn send_message<M: ReplyMarkup>(
        &self,
        chat_id: &i64,
        text: &str,
        parse_mode: Option<&ParseMode>,
        disable_web_page_preview: Option<&bool>,
        disable_notification: Option<&bool>,
        reply_to_message_id: Option<&i64>,
        reply_markup: Option<M>,
    ) -> Result<Message> {
        debug!("Calling send_message...");
        let chat_id: &str = &chat_id.to_string();
        let parse_mode = &get_parse_mode(parse_mode.unwrap_or(&ParseMode::Text));
        let disable_web_page_preview: &str =
            &disable_web_page_preview.unwrap_or(&false).to_string();
        let disable_notification: &str = &disable_notification.unwrap_or(&false).to_string();
        let reply_to_message_id: &str = &reply_to_message_id.map(|i| i.to_string()).unwrap_or(
            "None"
                .to_string(),
        );
        let reply_markup = &Box::new(reply_markup)
            .map(|r| serde_json::to_string(&r).unwrap_or("".to_string()))
            .unwrap_or("".to_string());

        let path = ["sendMessage"];
        let params = [
            ("chat_id", chat_id),
            ("text", text),
            ("parse_mode", parse_mode),
            ("disable_web_page_preview", disable_web_page_preview),
            ("disable_notification", disable_notification),
            ("reply_to_message_id", reply_to_message_id),
            ("reply_markup", reply_markup),
        ];
        self.call(&path, &params)
    }

    /// API call which will reply to a message directed to your bot.
    pub fn reply_to_message(&self, update: &Update, text: &str) -> Result<Message> {
        debug!("Calling reply_to_message...");
        let message = update.clone().message.unwrap();
        let message_id = message.message_id;
        let chat_id = message.chat.id;
        self.send_message(
            &chat_id,
            text,
            None,
            None,
            None,
            Some(&message_id),
            ::NO_MARKUP,
        )
    }

    /// API call which will forward a message.
    pub fn forward_message(
        &self,
        update: &Update,
        chat_id: &i64,
        disable_notification: Option<&bool>,
    ) -> Result<Message> {
        debug!("Calling forward_message...");
        let message = update.clone().message.unwrap();
        let chat_id: &str = &chat_id.to_string();
        let from_chat_id: &str = &message.chat.id.to_string();
        let message_id: &str = &message.message_id.to_string();
        let disable_notification: &str = &disable_notification.unwrap_or(&false).to_string();
        let path = ["forwardMessage"];
        let params = [
            ("chat_id", chat_id),
            ("from_chat_id", from_chat_id),
            ("disable_notification", disable_notification),
            ("message_id", message_id),
        ];
        self.call(&path, &params)
    }

    /// API call which will show the given chat action to the users.
    pub fn send_chat_action(&self, chat_id: &i64, action: &ChatAction) -> Result<bool> {
        debug!("Calling send_chat_action...");
        let chat_id: &str = &chat_id.to_string();
        let action = &get_chat_action(action);
        let path = ["sendChatAction"];
        let params = [("chat_id", chat_id), ("action", action)];
        self.call(&path, &params)
    }

    /// API call which will send the given contact.
    pub fn send_contact<M: ReplyMarkup>(
        &self,
        chat_id: &i64,
        contact: &Contact,
        disable_notification: Option<&bool>,
        reply_to_message_id: Option<&i64>,
        reply_markup: Option<M>,
    ) -> Result<Message> {
        debug!("Calling send_contact...");
        let chat_id: &str = &chat_id.to_string();
        let phone_number = &contact.phone_number;
        let first_name = &contact.first_name;
        let last_name = &contact.clone().last_name.unwrap();
        let disable_notification: &str = &disable_notification.unwrap_or(&false).to_string();
        let reply_to_message_id: &str = &reply_to_message_id.map(|i| i.to_string()).unwrap_or(
            "None"
                .to_string(),
        );
        let reply_markup = &Box::new(reply_markup)
            .map(|r| serde_json::to_string(&r).unwrap_or("".to_string()))
            .unwrap_or("".to_string());
        let path = ["sendContact"];
        let params = [
            ("chat_id", chat_id),
            ("phone_number", phone_number),
            ("first_name", first_name),
            ("last_name", last_name),
            ("disable_notification", disable_notification),
            ("reply_to_message_id", reply_to_message_id),
            ("reply_markup", reply_markup),
        ];
        self.call(&path, &params)
    }

    /// API call which will get a list of profile pictures for a user.
    pub fn get_user_profile_photos(
        &self,
        user_id: &i64,
        offset: Option<&i64>,
        limit: Option<&i64>,
    ) -> Result<UserProfilePhotos> {
        debug!("Calling get_user_profile_photos...");
        let user_id: &str = &user_id.to_string();
        let offset: &str = &offset.map(|i| i.to_string()).unwrap_or("None".to_string());
        let limit: &str = &limit.map(|i| i.to_string()).unwrap_or("None".to_string());
        let path = ["getUserProfilePhotos"];
        let params = [("user_id", user_id), ("offset", offset), ("limit", limit)];
        self.call(&path, &params)
    }

    /// API call which will get basic info about a file and prepare it for donwloading.
    pub fn get_file(&self, file_id: &str) -> Result<File> {
        debug!("Calling get_file...");
        let path = ["getFile"];
        let params = [("file_id", file_id)];
        self.call(&path, &params)
    }

    /// API call which will kick a user from a group, supergroup or a channel. The bot must be
    /// an administrator in the chat and must have the appropiate rights for this call to succeed.
    pub fn kick_chat_member(
        &self,
        chat_id: &i64,
        user_id: &i64,
        until_date: Option<i64>,
    ) -> Result<bool> {
        debug!("Calling kick_chat_member...");
        let chat_id: &str = &chat_id.to_string();
        let user_id: &str = &user_id.to_string();
        let until_date: &str = &until_date.map(|i| i.to_string()).unwrap_or(
            "None".to_string(),
        );
        let path = ["kickChatMember"];
        let params = [
            ("chat_id", chat_id),
            ("user_id", user_id),
            ("until_date", until_date),
        ];
        self.call(&path, &params)
    }

    /// API call which will unban previously kicked members in a supergroup or channel. The bot
    /// must be an administrator and must have the appropiate rights for this call to succeed.
    pub fn unban_chat_member(&self, chat_id: &i64, user_id: &i64) -> Result<bool> {
        debug!("Calling unban_chat_member...");
        let chat_id: &str = &chat_id.to_string();
        let user_id: &str = &user_id.to_string();
        let path = ["unbanChatMember"];
        let params = [("chat_id", chat_id), ("user_id", user_id)];
        self.call(&path, &params)
    }

    /// API call which will export an invite link to a supergroup or channel. The bot must be an
    /// administrator and must have the the appropiate rights for this call to succeed.
    pub fn export_chat_invite_link(&self, chat_id: &i64) -> Result<String> {
        debug!("Calling export_chat_invite_link...");
        let chat_id: &str = &chat_id.to_string();
        let path = ["exportChatInviteLink"];
        let params = [("chat_id", chat_id)];
        self.call(&path, &params)
    }

    /// The actual networking done for making API calls.
    fn call<T: DeserializeOwned>(&self, path: &[&str], params: &[(&str, &str)]) -> Result<T> {
        debug!("Making API call...");
        let url = ::construct_api_url(&self.bot_url, path);
        let mut data = self.client.post(&url)?.form(&params)?.send()?;
        let rjson: Value = check_for_error(data.json()?)?;
        let object_json = rjson.get("result").ok_or(JsonNotFound)?;
        let object = serde_json::from_value::<T>(object_json.clone())?;
        Ok(object)
    }
}
