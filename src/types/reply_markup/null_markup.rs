use marker::ReplyMarkup;

/// Empty Markup placeholder.
#[derive(Debug, Default, Serialize)]
pub struct NullMarkup;

impl ReplyMarkup for NullMarkup {}
