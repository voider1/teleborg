use crate::types::{
    CallbackQuery, ChosenInlineResult, InlineQuery, Message, Poll, PollAnswer, PreCheckoutQuery,
    ShippingQuery,
};
use serde::Deserialize;

/// This object represents an incoming update.At most one of the optional parameters can be present in any given update.
#[derive(Clone, Deserialize, Debug)]
pub struct Update {
    /// The update‘s unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you’re using Webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// Optional. New incoming message of any kind — text, photo, sticker, etc.
    pub message: Option<Message>,
    /// Optional. New version of a message that is known to the bot and was edited
    pub edited_message: Option<Message>,
    /// Optional. New incoming channel post of any kind — text, photo, sticker, etc.
    pub channel_post: Option<Message>,
    /// Optional. New version of a channel post that is known to the bot and was edited
    pub edited_channel_post: Option<Message>,
    /// Optional. New incoming inline query
    pub inline_query: Option<InlineQuery>,
    /// Optional. The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
    pub chosen_inline_result: Option<ChosenInlineResult>,
    /// Optional. New incoming callback query
    pub callback_query: Option<CallbackQuery>,
    /// Optional. New incoming shipping query. Only for invoices with flexible price
    pub shipping_query: Option<ShippingQuery>,
    /// Optional. New incoming pre-checkout query. Contains full information about checkout
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    /// Optional. New poll state. Bots receive only updates about stopped polls and polls, which are sent by the bot
    pub poll: Option<Poll>,
    /// Optional. A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
    pub poll_answer: Option<PollAnswer>,
}
