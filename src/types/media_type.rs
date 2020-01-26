//use std::path::Path;

#[allow(dead_code)]
/// type that must be past with file sending methods
#[derive(Debug)]
pub enum MediaType {
    /// Send Url
    Url(String),
    /// Send File id
    FileId(String),
    //:Multipart(&Path),
}
