use super::Method;
use types::UserProfilePhotos;

#[derive(Debug, Builder, Serialize)]
pub struct GetUserProfilePhotos {
    user_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")] offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")] limit: Option<i32>,
}

impl Method for GetUserProfilePhotos {
    type Response = UserProfilePhotos;
    const PATH: &'static str = "getUserProfilePhotos";
}
