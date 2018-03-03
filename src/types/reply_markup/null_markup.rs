use marker::ReplyMarkup;

/// Empty Markup placeholder.
#[derive(Debug, Default, Clone, Serialize)]
pub struct NullMarkup;

impl ReplyMarkup for NullMarkup {}
