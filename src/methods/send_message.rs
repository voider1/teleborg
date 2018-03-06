use super::Method;
use types::{Message, ParseMode};
use marker::ReplyMarkup;

#[derive(Debug, Builder, Serialize)]
pub struct SendMessage<M: ReplyMarkup + Default> {
    chat_id: i64,
    text: &'static str,
    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_view: Option<bool>,
    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i32>,
    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<M>,
}

impl<M: ReplyMarkup + Default> SendMessage<M> {
    pub fn builder() -> SendMessageBuilder<M> {
        SendMessageBuilder::default()
    }
}

impl<M: ReplyMarkup + Default> Method for SendMessage<M> {
    type Response = Message;
    const PATH: &'static str = "sendMessage";
}
