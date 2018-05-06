use super::PhotoSize;

/// This struct represents a user's profile pictures.
#[derive(Clone, Deserialize, Debug)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has.
    pub total_count: i64,
    /// Requested profile pictures.
    pub photos: Vec<Vec<PhotoSize>>,
}
