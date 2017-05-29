//! # teleborg
//!
//! The `teleborg` library provides an interface to the [Telegram bot
//! API](https://core.telegram.org/bots/api).
//!
//! ## Getting started
//!
//! ``` no_run
//! extern crate teleborg;
//! use teleborg::{Dispatcher, Bot, Updater};
//! use teleborg::objects::Update;
//!
//! fn main() {
//!     // Make sure you have your token
//!     let bot_token = "bot_token".to_string();
//!     // Creating a dispatcher which registers all the command and message handlers
//!     let mut dispatcher = Dispatcher::new();
//!     // Registering our command which we create below in the form as a function
//!     dispatcher.add_command_handler("test", test, false);
//!     // Start the updater, the Updater will start the threads, one of which will poll for updates
//!     // and send those to the Dispatcher's thread which will act upon it with the registered handlers
//!     Updater::start(Some(bot_token), None, None, None, dispatcher);
//! }
//!
//! // Our first command handler
//! fn test(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
//!     bot.reply_to_message(&update, "It works!").unwrap();
//! }
//! ```


use std::marker::{Sync, Send};

#[macro_use]
extern crate log;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rand;

pub use reqwest::StatusCode;

pub use self::bot::{Bot, ParseMode, ChatAction};
pub use self::dispatcher::Dispatcher;
pub use self::updater::Updater;
pub use self::command::Command;

mod updater;
mod dispatcher;
mod command;
mod value_extension;
mod bot;
pub mod objects;
pub mod error;

impl<T: Sync + Send + 'static + FnMut(&Bot, objects::Update, Option<Vec<&str>>)> Command for T {
    fn execute(&mut self, bot: &Bot, update: objects::Update, args: Option<Vec<&str>>) {
        self(bot, update, args);
    }
}

/// Construct an API URL with the base bot URL and an
/// array of strings which will construct the path.
fn construct_api_url(bot_url: &str, path: &[&str]) -> String {
    format!("{}/{}", bot_url, path.join("/"))
}
