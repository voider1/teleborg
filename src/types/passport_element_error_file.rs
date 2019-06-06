use serde::Deserialize;

/// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
#[derive(Clone, Deserialize, Debug)]
pub struct PassportElementErrorFile {
    /// Error source, must be file
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub kind: String,
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String,
}
