use super::InlineQueryResultType;

use std::any::Any;

pub trait InlineQueryResult {
    fn get_type(&self) -> InlineQueryResultType;
    fn as_any(&self) -> &Any;
}
