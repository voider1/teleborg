use objects::PhotoSize;

/// Represents a sticker
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Sticker {
    pub file_id: String,
    pub width: i64,
    pub heigh: i64,
    pub thump: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub file_size: Option<i64>,
}
