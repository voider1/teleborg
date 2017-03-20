use serde_json;
use serde_json::Value;

use bot::parse_mode::{get_parse_mode, ParseMode};
use error::Error::{JsonNotFound};
use error::{Result, check_for_error};
use objects::{Update, Message};
use bot::Bot;

impl Bot {
    /// API call which will send a message to a chat which your bot participates in.
    pub fn send_message(&self,
                        chat_id: &i64,
                        text: &str,
                        parse_mode: Option<&ParseMode>,
                        disable_web_page_preview: Option<&bool>,
                        disable_notification: Option<&bool>,
                        reply_to_message_id: Option<&i64>)
                        -> Result<Message> {
        let chat_id: &str = &chat_id.to_string();
        let parse_mode = &get_parse_mode(parse_mode.unwrap_or(&ParseMode::Text));
        let disable_web_page_preview: &str = &disable_web_page_preview.unwrap_or(&false)
            .to_string();
        let disable_notification: &str = &disable_notification.unwrap_or(&false).to_string();
        let reply_to_message_id: &str = &reply_to_message_id.map(|i| i.to_string())
            .unwrap_or("None".to_string());
        let path = ["sendMessage"];
        let params = [("chat_id", chat_id),
                      ("text", text),
                      ("parse_mode", parse_mode),
                      ("disable_web_page_preview", disable_web_page_preview),
                      ("disable_notification", disable_notification),
                      ("reply_to_message_id", reply_to_message_id)];
        self.post_message(&path, &params)
    }

    /// API call which will reply to a message directed to your bot.
    pub fn reply_to_message(&self, update: &Update, text: &str) -> Result<Message> {
        let message = update.clone().message.unwrap();
        let message_id = message.message_id;
        let chat_id = message.chat.id;
        self.send_message(&chat_id, text, None, None, None, Some(&message_id))
    }

    /// API call which will forward a message.
    pub fn forward_messge(&self,
                          update: &Update,
                          chat_id: &i32,
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
