use super::Method;
use types::UserProfilePhotos;

#[derive(Debug, Builder, Serialize)]
pub struct GetUserProfilePhotos {
    user_id: i32,
    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<i32>,
    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
}

impl_builder!(GetUserProfilePhotos, GetUserProfilePhotosBuilder);
impl_method!(
    GetUserProfilePhotos,
    UserProfilePhotos,
    "getUserProfilePhotos"
);
