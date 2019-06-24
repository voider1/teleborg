#![deny(warnings)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

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
//! use std::{sync::Arc, env};
//! use teleborg::{methods::SendMessage, spawn, types::Update, Bot, Dispatcher, Future, Updater,
//! HasArgs};
//!
//! fn main() {
//!     // Get bot your token from the environment
//!     let token = env::var("TELEGARM_BOT_TOKEN").expect("No token found");
//!     // Creating a dispatcher which registers all the command and message handlers
//!     let mut dispatcher = Dispatcher::new();
//!     // Registering our command which we create below in the form as a function
//!     dispatcher.add_command_handler("test", test, HasArgs::No);
//!     // Create an Updater builder and configure it as you like, after that build it and start it.
//!     // The Updater will start the Tokio runtime, this ensures you can spawn tasks inside of
//!     // your command handlers.
//!     Updater::builder().build(&token).start(dispatcher);
//! }
//!
//! // Our first command handler
//! fn test(bot: &Arc<Bot>, update: Update, _: Option<Vec<&str>>) {
//!     let chat_id = update.message.unwrap().chat.id;
//!     let text = String::from("It works!");
//!
//!     let msg = SendMessage::builder().chat_id(chat_id).text(text).build();
//!
//!     spawn(bot.call(msg).then(|_| Ok(())));
//! }
//!
//! ```
//!
//! ## Sending files
//!
//! Since some methods require to be able to send `files`.
//! We need to be able to either send an `file` or an `file_id` or `file_url`.
//! To do this we have created a field for the method builder called 'file' which you can call by
//! using `.file()` with the file path as parameter.
//!
//! ``` no_run
//! fn test(bot: &Arc<Bot>, update: Update, _: Option<Vec<&str>>) {
//!     let chat_id = update.message.unwrap().chat.id;
//!
//!     let msg = SendPhoto::builder().chat_id(chat_id).file("photos/crab.png");
//!     // example url/file_id sending
//!     // let msg = SendPhoto::builder().chat_id(chat_id).photo("https://example.com/photo.png");
//!     
//!     spawn(bot.call(msg).then(|_| Ok(())));
//!
//! }
//! ```
//!

pub use self::{
    bot::Bot,
    command::Command,
    dispatcher::{Dispatcher, HasArgs},
    methods::Method,
    updater::Updater,
};
pub use futures::Future;
use std::sync::Arc;
pub use tokio::spawn;

/// This module contains all the error-types.
pub mod error;
/// This module contains all the method-builders.
#[macro_use]
pub mod methods;
mod bot;
mod command;
mod dispatcher;
/// This module contains all the types which you can send an receive.
pub mod types;
mod updater;

impl<T> Command for T
where
    T: Sync + Send + 'static + FnMut(&Arc<Bot>, types::Update, Option<Vec<&str>>),
{
    fn execute(&mut self, bot: &Arc<Bot>, update: types::Update, args: Option<Vec<&str>>) {
        self(bot, update, args);
    }
}

/// Construct an API URL with the base bot URL and an
/// array of strings which will construct the path.
fn construct_api_url(bot_url: &str, path: &[&str]) -> String {
    format!("{}/{}", bot_url, path.join("/"))
}
