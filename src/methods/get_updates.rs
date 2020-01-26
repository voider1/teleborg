/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use super::Method;
use crate::types::{Update};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to receive incoming updates using long polling (wiki). An Array of Update objects is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct GetUpdates {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifier of the first update to be returned. Must be greater by one than the
    pub offset: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limits the number of updates to be retrieved. Values between 1—100 are accepted.
    pub limit: Option<u8>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling.
    pub timeout: Option<u32>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List the types of updates you want your bot to receive. For example, specify
    pub allowed_updates: Option<Vec<String>>,
}

impl_method!(GetUpdates, Vec<Update>);
