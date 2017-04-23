use objects::PhotoSize;

/// Represents a PhotoSize object
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PhotoSize {
    pub file_id: String,
    pub thump: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}
