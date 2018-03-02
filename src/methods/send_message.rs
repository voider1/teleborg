use super::Method;
use types::{ReplyMarkup, Message};

#[derive(Debug, Serialize, Builder)]
pub struct<M: ReplyMarkup> SendMessage {
    chat_id: i32,
    text: &'static str,
    parse_mode: Option<&'static str>,
    disable_web_page_view: Option<bool>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i32>,
    reply_markup: Option<M>,
}

impl Method for SendMessage {
    type Response = Message;
    const URL: &'static str = "";
}