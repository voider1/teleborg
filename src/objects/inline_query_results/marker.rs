use super::InlineQueryResultType;

use std::any::Any;

/// Trait which is implemented for all inline query result types.
pub trait InlineQueryResult {
    fn get_type(&self) -> InlineQueryResultType;
    fn as_any(&self) -> &Any;
}
