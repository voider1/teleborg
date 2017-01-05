#![feature(proc_macro)]

extern crate chrono;
extern crate crossbeam;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub use reqwest::StatusCode;
use chrono::NaiveDateTime;
use serde_json::Value;

mod bot;
mod chat;
mod message;
mod update;
mod error;
mod user;
mod message_entity;
pub mod updater;

use error::{Error, Result};

pub trait Plugin {
    fn execute(&mut self, update::Update);
}

trait ValueExtension {
    fn as_optional_string(&self, field: &str) -> Option<String>;

    fn as_required_string(&self, field: &str) -> Result<String>;

    fn as_optional_i64(&self, field: &str) -> Option<i64>;

    fn as_required_i64(&self, field: &str) -> Result<i64>;

    fn as_optional_bool(&self, field: &str) -> Option<bool>;

    fn as_optional_date(&self, field: &str) -> Option<NaiveDateTime>;

    fn as_required_date(&self, field: &str) -> Result<NaiveDateTime>;
}

impl<T: FnMut(update::Update)> Plugin for T {
    fn execute(&mut self, update: update::Update) {
        self(update);
    }
}

impl ValueExtension for Value {
    fn as_optional_string(&self, field: &str) -> Option<String> {
        self.find(field).and_then(|v| v.as_str()).map(|s| s.to_owned())
    }

    fn as_required_string(&self, field: &str) -> Result<String> {
        self.find(field).and_then(|v| v.as_str()).map(|s| s.to_owned()).ok_or(Error::JsonNotFound)
    }

    fn as_optional_i64(&self, field: &str) -> Option<i64> {
        self.find(field).and_then(|v| v.as_i64())
    }

    fn as_required_i64(&self, field: &str) -> Result<i64> {
        self.find(field).and_then(|v| v.as_i64()).ok_or(Error::JsonNotFound)
    }

    fn as_optional_date(&self, field: &str) -> Option<NaiveDateTime> {
        self.as_optional_i64(field).map(|v| NaiveDateTime::from_timestamp(0, v as u32))
    }

    fn as_optional_bool(&self, field: &str) -> Option<bool> {
        self.find(field).and_then(|v| v.as_bool())
    }

    fn as_required_date(&self, field: &str) -> Result<NaiveDateTime> {
        Ok(NaiveDateTime::from_timestamp(0, self.as_required_i64(field)? as u32))
    }
}

/// Construct an API URL with the base bot URL and an
/// array of strings which will construct the path.
fn construct_api_url(bot_url: &str, path: &[&str]) -> String {
    format!("{}/{}", bot_url, path.join("/"))
}
