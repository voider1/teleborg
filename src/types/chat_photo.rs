use serde::Deserialize;

/// This struct represents a chat photo.
#[derive(Clone, Deserialize, Debug)]
pub struct ChatPhoto {
    /// Unique file identifier of the small (160x160) chat photo. This file_id can be used only for
    /// photo download.
    pub small_file_id: String,
    /// Unique file identifier of the big (640x640) chat photo. This file_id can be used only for
    /// photo download.
    pub big_file_id: String,
}
