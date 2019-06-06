use serde::Deserialize;
use crate::types::{PhotoSize};

/// This object represent a user's profile pictures.
#[derive(Clone, Deserialize, Debug)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub total_count: i64,
    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Vec<PhotoSize>,
}

