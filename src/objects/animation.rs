use objects::PhotoSize;

/// Represents an animation object.
#[derive(Clone, Deserialize, Debug)]
pub struct Animation {
    pub file_id: String,
    pub thump: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}
