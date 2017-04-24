use objects::PhotoSize;

/// Represents a document.
#[derive(Clone, Deserialize, Debug)]
pub struct Document {
    pub file_id: String,
    pub thump: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_typ: Option<String>,
    pub file_size: Option<i64>,
}
