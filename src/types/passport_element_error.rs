use crate::types::{InputMessageContent, ParseMode, ReplyMarkup};
use serde::Deserialize;

/// This object represents one result of an inline query. Telegram clients currently support results of the following 20 types.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum PassportElementError {
    /// Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.
    DataField {
        /// Error source, must be data
        pub source: String,
        /// The section of the user's Telegram Passport which has the error, one of “personal_details”, “passport”, “driver_license”, “identity_card”, “internal_passport”, “address”
        #[serde(rename = "type")]
        pub kind: String,
        /// Name of the data field which has the error
        pub field_name: String,
        /// Base64-encoded data hash
        pub data_hash: String,
        /// Error message
        pub message: String,
    },
    /// Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
    FrontSide {
        /// Error source, must be front_side
        pub source: String,
        /// The section of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”
        #[serde(rename = "type")]
        pub kind: String,
        /// Base64-encoded hash of the file with the front side of the document
        pub file_hash: String,
        /// Error message
        pub message: String,
    },
    /// Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
    ReverseSide {
        /// Error source, must be reverse_side
        pub source: String,
        /// The section of the user's Telegram Passport which has the issue, one of ‚Äúdriver_license‚Äù, ‚Äúidentity_card‚Äù
        #[serde(rename = "type")]
        pub kind: String,
        /// Base64-encoded hash of the file with the reverse side of the document
        pub file_hash: String,
        /// Error message
        pub message: String,
    },
    /// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
    Selfie {
        /// Error source, must be selfie
        pub source: String,
        /// The section of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”
        #[serde(rename = "type")]
        pub kind: String,
        /// Base64-encoded hash of the file with the selfie
        pub file_hash: String,
        /// Error message
        pub message: String,
    },
    /// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
    File {
        /// Error source, must be file
        pub source: String,
        /// The section of the user's Telegram Passport which has the issue, one of “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
        #[serde(rename = "type")]
        pub kind: String,
        /// Base64-encoded file hash
        pub file_hash: String,
        /// Error message
        pub message: String,
    },
    /// Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
    Files {
        /// Error source, must be files
        pub source: String,
        /// The section of the user's Telegram Passport which has the issue, one of “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
        #[serde(rename = "type")]
        pub kind: String,
        /// List of base64-encoded file hashes
        pub file_hashes: Vec<String>,
        /// Error message
        pub message: String,
    },
    /// Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.
    TranslationFile {
        /// Error source, must be translation_file
        pub source: String,
        /// Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
        #[serde(rename = "type")]
        pub kind: String,
        /// Base64-encoded file hash
        pub file_hash: String,
        /// Error message
        pub message: String,
    },
    /// Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
    TranslationFiles {
        /// Error source, must be translation_files
        pub source: String,
        /// Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
        #[serde(rename = "type")]
        pub kind: String,
        /// List of base64-encoded file hashes
        pub file_hashes: Vec<String>,
        /// Error message
        pub message: String,
    },
    /// Represents an issue in an unspecified place. The error is considered resolved when new data is added.
    Unspecified {
        /// Error source, must be unspecified
        pub source: String,
        /// Type of element of the user's Telegram Passport which has the issue
        #[serde(rename = "type")]
        pub kind: String,
        /// Base64-encoded element hash
        pub element_hash: String,
        /// Error message
        pub message: String,
    },
}
