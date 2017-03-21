use serde_json;
use serde_json::Value;

use error::Error::{JsonNotFound};
use error::{Result, check_for_error};
use objects::{InlineKeyboardMarkup, Contact};
use bot::Bot;

impl Bot {
    /// API call which will send the given contact.
    pub fn send_contact(&self,
                        chat_id: &i64,
                        contact: &Contact,
                        disable_notification: Option<&bool>,
                        reply_to_message_id: Option<&i32>,
                        reply_markup: Option<&InlineKeyboardMarkup>) -> Result<bool> {
        let chat_id: &str = &chat_id.to_string();
        let disable_notification: &str = &disable_notification.unwrap_or(&false).to_string();
        let reply_to_message_id: &str = &reply_to_message_id.map(|i| i.to_string())
            .unwrap_or("None".to_string());

        let reply_markup: &str = &if let Some(r) = reply_markup {
            serde_json::to_string(&reply_markup).unwrap_or("None".to_string())
        } else {
            "".to_string()
        };

        let path = ["sendContact"];
        let url = ::construct_api_url(&self.bot_url, &path);
        let params = [("chat_id", chat_id),
            ("phone_number", &contact.phone_number),
            ("first_name", &contact.first_name),
            ("last_name", &contact.clone().last_name.unwrap()),
            ("disable_notification", disable_notification),
            ("reply_to_message_id", reply_to_message_id),
            ("reply_markup", reply_markup)];
        let mut data = self.client.post(&url).form(&params).send()?;

        let rjson: Value = check_for_error(data.json()?)?;
        let message_json = rjson.get("result").ok_or(JsonNotFound)?;

        Ok(message_json.as_bool().unwrap_or(false))
    }
}