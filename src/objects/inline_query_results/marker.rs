extern crate serde;
use self::serde::Serialize;

pub trait InlineQueryResult: Serialize {}
