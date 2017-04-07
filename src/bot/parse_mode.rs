/// The parse modes for messages.
pub enum ParseMode {
    Markdown,
    Html,
    Text,
}

/// Does pattern matching on the `ParseMode` and gets the right `String`.
pub fn get_parse_mode(parse_mode: &ParseMode) -> String {
    match parse_mode {
            &ParseMode::Text => "None",
            &ParseMode::Markdown => "Markdown",
            &ParseMode::Html => "HTML",
        }
        .to_string()
}
