use serde::ser::{Serialize, Serializer};

/// This enum represents every possible chat action.
#[derive(Debug, Clone, Copy)]
pub enum ChatAction {
    /// Sets the "typing" chat action.
    Typing,
    /// Sets the "uploading photo" chat action.
    UploadPhoto,
    /// Sets the "recording video" chat action.
    RecordVideo,
    /// Sets the "upload video" chat action.
    UploadVideo,
    /// Sets the "uploading audio" chat action.
    UploadAudio,
    /// Sets the "uploading document" chat action.
    UploadDocument,
    /// Sets the "finding location" chat action.
    FindLocation,
}

impl Serialize for ChatAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let chat_action = match *self {
            ChatAction::Typing => "typing",
            ChatAction::UploadPhoto => "upload_photo",
            ChatAction::RecordVideo => "record_video",
            ChatAction::UploadVideo => "upload_video",
            ChatAction::UploadAudio => "upload_audio",
            ChatAction::UploadDocument => "upload_document",
            ChatAction::FindLocation => "find_location",
        };

        serializer.serialize_str(chat_action)
    }
}
