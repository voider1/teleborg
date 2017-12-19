use super::PhotoSize;

#[derive(Clone, Deserialize, Debug)]
pub struct UserProfilePhotos {
    total_count: i64,
    photos: Vec<Vec<PhotoSize>>,
}
