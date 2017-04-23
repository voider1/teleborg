/// Represents a PhotoSize object
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_path: Option<String>,
    pub width: i64,
    pub height: i64,
    pub file_size: Option<i64>,
}
