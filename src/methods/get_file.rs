/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use super::Method;
use crate::types::{File};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Use this method to get basic info about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a File object is returned. The file can then be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>, where <file_path> is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile again.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct GetFile {
    /// File identifier to get info about
    pub file_id: String,
}

impl_method!(GetFile, File);
