#![deny(warnings)]
#![deny(missing_docs)]

//! # Teleborg
//!
//! Teleborg is a fast, reliable and easy to use wrapper for the [Telegram bot
//! API](https://core.telegram.org/bots/api).
//! This crate aims to provide everything the user needs to create a high
//! performant Telegram bot.
//!
//! ## Getting started
//!
//! ``` no_run
//! extern crate teleborg;
//!
//! use std::env;
//!
//! use teleborg::{Bot, Dispatcher, Updater};
//! use teleborg::types::Update;
//! use teleborg::methods::{Method, SendMessage};
//!
//! fn main() {
//!     // Get bot your token from the environment
//!     let token = env::var("TELEGARM_BOT_TOKEN").expect("No token found");
//!     // Creating a dispatcher which registers all the command and message handlers
//!     let mut dispatcher = Dispatcher::new();
//!     // Registering our command which we create below in the form as a function
//!     dispatcher.add_command_handler("test", test, false);
//!     // Create an Updater builder and configure it as you like, after that build it and start it.
//!     // The Updater will start the threads, one of which will poll for updates
//!     // and send those to the Dispatcher's thread which will act upon it with the registered handlers
//!     Updater::builder().token(token).build().start(dispatcher);
//! }
//!
//! // Our first command handler
//! fn test(bot: &Bot, update: Update, _: Option<Vec<&str>>) {
//!     let chat_id = update.message.unwrap().chat.id;
//!     let text = "It works!";
//!
//!     SendMessage::builder()
//!         .chat_id(chat_id)
//!         .text(text)
//!         .build()
//!         .call(&bot)
//!         .unwrap();
//! }
//!
//! ```

#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate typed_builder;

pub use self::bot::Bot;
pub use self::command::Command;
pub use self::dispatcher::Dispatcher;
pub use self::updater::Updater;
pub use self::methods::Method;

/// This module contains all the error-types.
pub mod error;
/// This module contains all the method-builders.
#[macro_use]
pub mod methods;
/// This module contains all the types which you can send an receive.
pub mod types;
mod bot;
mod command;
mod dispatcher;
mod updater;

impl<T: Sync + Send + 'static + FnMut(&Bot, types::Update, Option<Vec<&str>>)> Command for T {
    fn execute(&mut self, bot: &Bot, update: types::Update, args: Option<Vec<&str>>) {
        self(bot, update, args);
    }
}

/// Construct an API URL with the base bot URL and an
/// array of strings which will construct the path.
fn construct_api_url(bot_url: &str, path: &[&str]) -> String {
    format!("{}/{}", bot_url, path.join("/"))
}
