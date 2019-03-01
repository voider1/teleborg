use std::sync::Arc;

use crate::types::Update;
use crate::Bot;

/// A trait which has to be implemented for all handlers.
///
/// With this trait you can create your own custom commands.
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
///
/// use teleborg::{Command, methods::SendMessage, spawn, types::Update, Bot, Future};
///
/// struct Test;
///
/// impl Test {
///     fn test(&self, bot: &Arc<Bot>, update: Update, _: Option<Vec<&str>>) {
///         let chat_id = update.message.unwrap().chat.id;
///         let text = String::from("It works!");
///
///         let msg = SendMessage::builder()
///             .chat_id(chat_id)
///             .text(text)
///             .build();
///
///         spawn(bot.call(&msg).then(|_| Ok(())));
///     }
/// }
///
/// impl Command for Test {
///     fn execute(&mut self, bot: &Arc<Bot>, update: Update, args: Option<Vec<&str>>) {
///         self.test(bot, update, args);
///     }
/// }
/// ```
///
/// This implements the `Command` trait for the Test struct.
pub trait Command: Sync + Send + 'static {
    /// Execute the logic for each handler.
    fn execute(&mut self, bot: &Arc<Bot>, update: Update, args: Option<Vec<&str>>);
}
