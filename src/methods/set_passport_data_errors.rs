use super::Method;
use crate::types::{PassportElementError};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SetPassportDataErrors {
    /// User identifier
    pub user_id: i64,
    /// A JSON-serialized array describing the errors
    pub errors: Vec<PassportElementError>,
}

impl_method!(SetPassportDataErrors, bool, "setPassportDataErrors");