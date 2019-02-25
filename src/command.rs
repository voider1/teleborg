use crate::types::Update;
use crate::Bot;

/// A trait which has to be implemented for all handlers.
///
/// With this trait you can create your own custom commands.
///
/// # Examples
///
/// ```
/// use teleborg::{Bot, Command};
/// use teleborg::objects::Update;
///
/// struct Test;
///
/// impl Test {
///     fn test(&self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
///         bot.reply_to_message(&update, "It works!");
///     }
/// }
///
/// impl Command for Test {
///     fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
///         self.test(bot, update, args);
///     }
/// }
/// ```
///
/// This implements the `Command` trait for the Test struct.
pub trait Command: Sync + Send + 'static {
    /// Execute the logic for each handler.
    fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>);
}
