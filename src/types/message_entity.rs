use crate::types::User;
/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use serde::Deserialize;

/// This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
#[derive(Clone, Deserialize, Debug)]
pub struct MessageEntity {
    /// Type of the entity. Can be “mention” (@username), “hashtag” (#hashtag), “cashtag” ($USD), “bot_command” (/start@jobs_bot), “url” (https://telegram.org), “email” (do-not-reply@telegram.org), “phone_number” (+1-212-555-0123), “bold” (bold text), “italic” (italic text), “underline” (underlined text), “strikethrough” (strikethrough text), “code” (monowidth string), “pre” (monowidth block), “text_link” (for clickable text URLs), “text_mention” (for users without usernames)
    #[serde(rename = "type")]
    pub kind: String,
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: i64,
    /// Length of the entity in UTF-16 code units
    pub length: i64,
    /// Optional. For “text_link” only, url that will be opened after user taps on the text
    pub url: Option<String>,
    /// Optional. For “text_mention” only, the mentioned user
    pub user: Option<User>,
    /// Optional. For “pre” only, the programming language of the entity text
    pub language: Option<String>,
}
