use std::marker::{Send, Sync};

use objects::Update;
use bot;

/// A trait which has to be implemented for all handlers.
///
/// With this trait you can create your own custom commands.
///
/// # Examples
///
/// ```
/// impl<T: Sync + Send + 'static + FnMut(&bot::Bot, update::Update, Option<Vec<&str>>)> Command for T {
///     fn execute(&mut self, bot: &bot::Bot, update: update::Update, args: Option<Vec<&str>>) {
///         self(bot, update, args);
///     }
/// }
/// ```
///
/// This implements the `Command` trait for all functions which take a reference
/// to `Bot`, an `Update` and an optional `Vec` which contains `&str`.
pub trait Command: Sync + Send + 'static {
    fn execute(&mut self, bot: &bot::Bot, update: Update, args: Option<Vec<&str>>);
}
