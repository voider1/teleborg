use marker::ReplyMarkup;

#[derive(Serialize, Debug)]
pub struct NullMarkup;

impl ReplyMarkup for NullMarkup {}
