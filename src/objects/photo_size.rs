/// Represents a PhotoSize object, which is basically a photo.
#[derive(Clone, Deserialize, Debug)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_path: Option<String>,
    pub width: i64,
    pub height: i64,
    pub file_size: Option<i64>,
}
