/// Represents a voice memo.
#[derive(Clone, Deserialize, Debug)]
pub struct Voice {
    pub file_id: String,
    pub duration: i64,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}
