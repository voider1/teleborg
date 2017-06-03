use super::InputMessageContentType;

use std::any::Any;

/// Trait which is implemented for all input message content types.
pub trait InputMessageContent {
    fn as_any(&self) -> &Any;
    fn get_type(&self) -> InputMessageContentType;
}
