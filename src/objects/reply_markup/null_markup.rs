use marker::ReplyMarkup;

/// Empty Markup placeholder.
#[derive(Serialize, Debug)]
pub struct NullMarkup;

impl ReplyMarkup for NullMarkup {}
