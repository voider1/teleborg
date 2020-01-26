use serde::{Deserialize, Serialize};

/// The parse modes for messages.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ParseMode {
    /// Parse the text in a request as markdown.
    Markdown,
    /// Parse the text in a request as HTML.
    Html,
}
