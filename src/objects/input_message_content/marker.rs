use super::InputMessageType;

use std::any::Any;

pub trait InputMessageContent {
    fn as_any(&self) -> &Any;
    fn get_type(&self) -> InputMessageType;
}
