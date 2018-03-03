use super::Method;
use types::Message;
use marker::ReplyMarkup;

#[derive(Debug, Builder, Serialize)]
pub struct SendContact<M: ReplyMarkup + Default> {
    chat_id: i32,
    phone_number: &'static str,
    first_name: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")] last_name: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")] disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] reply_to_message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")] reply_markup: Option<M>,
}

impl<M: ReplyMarkup + Default> SendContact<M> {
    pub fn builder() -> SendContactBuilder<M> {
        SendContactBuilder::default()
    }
}

impl<M: ReplyMarkup + Default> Method for SendContact<M> {
    type Response = Message;
    const PATH: &'static str = "sendContact";
}
