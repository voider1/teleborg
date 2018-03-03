use super::Method;
use types::File;

#[derive(Debug, Builder, Serialize)]
pub struct GetFile {
    file_id: &'static str,
}

impl_builder!(GetFile, GetFileBuilder);
impl_method!(GetFile, File, "getFile");
