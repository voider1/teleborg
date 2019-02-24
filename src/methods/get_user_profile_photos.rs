use super::Method;
use crate::types::UserProfilePhotos;

use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to get a list of profile pictures for a user. Returns a `UserProfilePhotos`
/// struct instance.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct GetUserProfilePhotos {
    user_id: i32,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<i32>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
}

impl_method!(
    GetUserProfilePhotos,
    UserProfilePhotos,
    "getUserProfilePhotos"
);
