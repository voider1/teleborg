use std::marker::{Sync, Send};

extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub use reqwest::StatusCode;

pub use self::bot::{Bot, ParseMode};
pub use self::dispatcher::Dispatcher;
pub use self::updater::Updater;
pub use self::command::Command;
pub use objects::*;

mod updater;
mod dispatcher;
mod command;
mod value_extension;
mod bot;
mod objects;
pub mod error;

impl<T: Sync + Send + 'static + FnMut(&Bot, Update, Option<Vec<&str>>)> Command for T {
    fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        self(bot, update, args);
    }
}

/// Construct an API URL with the base bot URL and an
/// array of strings which will construct the path.
fn construct_api_url(bot_url: &str, path: &[&str]) -> String {
    format!("{}/{}", bot_url, path.join("/"))
}
