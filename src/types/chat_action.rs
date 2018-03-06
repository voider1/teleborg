use serde::ser::{Serialize, Serializer};

/// Represents each chat action.
#[derive(Debug, Clone, Copy)]
pub enum ChatAction {
    Typing,
    UploadPhoto,
    RecordVideo,
    UploadVideo,
    UploadAudio,
    UploadDocument,
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
