use std::marker::{Sync, Send};

extern crate crossbeam;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub use reqwest::StatusCode;

pub mod updater;
pub mod dispatcher;
pub mod command;
mod value_extension;
pub mod objects;
pub mod bot;
mod error;

use command::Command;
use objects::update;

impl<T: Sync + Send + 'static + FnMut(&bot::Bot, update::Update)> Command for T {
    fn execute(&mut self, bot: &bot::Bot, update: update::Update) {
        self(bot, update);
    }
}

/// Construct an API URL with the base bot URL and an
/// array of strings which will construct the path.
fn construct_api_url(bot_url: &str, path: &[&str]) -> String {
    format!("{}/{}", bot_url, path.join("/"))
}
