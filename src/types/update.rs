use super::{CallBackQuery, Message, ShippingQuery};

use serde::Deserialize;

/// This struct represents an incoming update. At most one of the optional parameters can be
/// present in any given update.
#[derive(Clone, Deserialize, Debug)]
pub struct Update {
    /// The update's unique identifier. Update identifiers start from a certain positive number
    /// and increase sequentially. This ID becomes especially handy if you’re using Webhooks,
    /// since it allows you to ignore repeated updates or to restore the correct update sequence,
    /// should they get out of order. If there are no new updates for at least a week, then
    /// identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// New incoming message of any kind.
    pub message: Option<Message>,
    /// New version of a message that is known to the bot and was edited.
    pub edited_message: Option<Message>,
    /// New incoming channel post of any kind.
    pub channel_post: Option<Message>,
    /// New version of a channel post that is known to the bot and was edited.
    pub edited_channel_post: Option<Message>,
    /// New incoming [inline](https://core.telegram.org/bots/api#inline-mode) query.
    pub inline_query: Option<String>,
    /// The result of an [inline](https://core.telegram.org/bots/api#inline-mode) query that was
    /// chosen by a user and sent to their chat partner.
    pub chosen_inline_result: Option<String>,
    /// New incomnig callback query.
    pub callback_query: Option<CallBackQuery>,
    /// New incoming shippinr query. Only for invoices with flexible price.
    pub shipping_query: Option<ShippingQuery>,
}
