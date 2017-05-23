extern crate serde;
use self::serde::{Deserialize, Serialize};

pub trait InputMessageContent: Serialize + Deserialize {}
