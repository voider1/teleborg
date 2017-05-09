extern crate serde;
use self::serde::{Deserialize, Serialize};

pub trait InlineQueryResult: Deserialize + Serialize {}
