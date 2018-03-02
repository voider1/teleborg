use super::Method;
use types::File;

#[derive(Debug, Builder, Serialize)]
pub struct GetFile {
    file_id: &'static str,
}

impl Method for GetFile {
    type Response = File;
    const PATH: &'static str = "getFile";
}
