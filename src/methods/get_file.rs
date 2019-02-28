use super::Method;
use crate::types::File;

use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to get basic information about a file and prepare it for downloading. For the
/// moment, bots can download files of up to 20MB in size. On success, a `File` struct instance is
/// returned. The file can be downloaded via the link
/// `https://api.telegram.org/file/bot<token>/<file_path>`, where `<file_path>` is taken from the
/// response. It is guaranteed that the link will be valid for at least 1 hour. When the link
/// expires, a new one can be requested by calling `GetFile` again.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct GetFile {
    /// File identifier to get information about.
    file_id: &'static str,
}

impl_method!(GetFile, File, "getFile");
