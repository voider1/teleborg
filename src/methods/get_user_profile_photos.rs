use super::Method;
use crate::types::UserProfilePhotos;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct GetUserProfilePhotos {
    /// Unique identifier of the target user
    pub user_id: i64,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sequential number of the first photo to be returned. By default, all photos are returned.
    pub offset: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limits the number of photos to be retrieved. Values between 1—100 are accepted. Defaults to 100.
    pub limit: Option<u8>,
}

impl_method!(GetUserProfilePhotos, UserProfilePhotos);
