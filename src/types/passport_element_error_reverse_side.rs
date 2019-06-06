use serde::Deserialize;

/// Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
#[derive(Clone, Deserialize, Debug)]
pub struct PassportElementErrorReverseSide {
    /// Error source, must be reverse_side
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “driver_license”, “identity_card”
    #[serde(rename = "type")]
    pub kind: String,
    /// Base64-encoded hash of the file with the reverse side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}

