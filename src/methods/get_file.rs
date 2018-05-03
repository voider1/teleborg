use super::Method;
use types::File;

#[derive(Debug, TypedBuilder, Serialize)]
pub struct GetFile {
    file_id: &'static str,
}

impl_method!(GetFile, File, "getFile");
