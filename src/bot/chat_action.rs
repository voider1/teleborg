/// Represents each chat action.
pub enum ChatAction {
    Typing,
    UploadPhoto,
    RecordVideo,
    UploadVideo,
    UploadAudio,
    UploadDocument,
    FindLocation,
}

/// Does pattern matching on the `ChatAction` and gets the right `String`.
pub fn get_chat_action(chat_action: &ChatAction) -> String {
    match chat_action {
            &ChatAction::Typing => "typing",
            &ChatAction::UploadPhoto => "upload_photo",
            &ChatAction::RecordVideo => "record_video",
            &ChatAction::UploadVideo => "upload_video",
            &ChatAction::UploadAudio => "upload_audio",
            &ChatAction::UploadDocument => "upload_document",
            &ChatAction::FindLocation => "find_location",
        }
        .to_string()
}
