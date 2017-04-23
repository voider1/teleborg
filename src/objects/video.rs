use objects::PhotoSize;

/// Represents a video file
#[derive(Clone, Deserialize, Debug)]
pub struct Video{
    pub file_id: String,
    pub width: i64,
    pub heigh: i64,
    pub duration: i64,
    pub thump: Option<PhotoSize>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}