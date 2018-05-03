use super::Method;
use types::UserProfilePhotos;

#[derive(Debug, TypedBuilder, Serialize)]
pub struct GetUserProfilePhotos {
    user_id: i32,
    #[default]
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<i32>,
    #[default]
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
}

impl_method!(
    GetUserProfilePhotos,
    UserProfilePhotos,
    "getUserProfilePhotos"
);
