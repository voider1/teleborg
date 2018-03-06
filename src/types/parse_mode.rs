use serde::ser::{Serialize, Serializer};

/// The parse modes for messages.
#[derive(Debug, Clone, Copy)]
pub enum ParseMode {
    Markdown,
    Html,
    Text,
}

impl Serialize for ParseMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let parse_mode = match *self {
            ParseMode::Markdown => "Markdown",
            ParseMode::Html => "Html",
            ParseMode::Text => "None",
        };

        serializer.serialize_str(parse_mode)
    }
}
