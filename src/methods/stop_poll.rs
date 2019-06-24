use super::Method;
use crate::types::{Poll, ReplyMarkup};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to stop a poll which was sent by the bot. On success, the stopped Poll with the final results is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct StopPoll {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Identifier of the original message with the poll
    pub message_id: i64,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for a new message inline keyboard.
    pub reply_markup: Option<ReplyMarkup>,
}

impl_method!(StopPoll, Poll, "stopPoll");
