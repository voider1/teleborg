use serde::Deserialize;

/// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
#[derive(Clone, Deserialize, Debug)]
pub struct PassportElementErrorSelfie {
    /// Error source, must be selfie
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”
    #[serde(rename = "type")]
    pub kind: String,
    /// Base64-encoded hash of the file with the selfie
    pub file_hash: String,
    /// Error message
    pub message: String,
}

