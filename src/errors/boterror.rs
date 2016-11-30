use std::io;

use reqwest;
use json;

#[derive(Debug)]
pub enum BotError {
	RequestError(reqwest::Error),
	IOError(io::Error),
	JsonError(json::Error),
	BadRequest(String),
}

impl From<reqwest::Error> for BotError {
	fn from (e: reqwest::Error) -> BotError {
		BotError::RequestError(e);
	}	
}

impl From<io::Error> for BotError {
	fn from (e: io::Error) -> BotError {
		BotError::IOError(e);
	}	
}

impl From<json::Error> for BotError {
	fn from (e: json::Error) -> BotError {
		BotError::JsonError(e);
	}	
}
