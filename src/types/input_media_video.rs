/// This code is generated using teleborg-api-generator (https://gitlab.com/b.wisman155/teleborg-api-generator)
use serde::Deserialize;use crate::types::{ParseMode};

/// Represents a video to be sent.
#[derive(Clone, Deserialize, Debug)]
pub struct InputMediaVideo {
    /// Type of the result, must be video
    #[serde(rename = "type")]
    pub kind: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More info on Sending Files »
    pub media: String,
    #[serde(skip_serializing)]
    /// thumb file to send with multipart
    pub thumb_file: Option<String>,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
    pub thumb: Option<String>,
    /// Optional. Caption of the video to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<ParseMode>,
    /// Optional. Video width
    pub width: Option<i64>,
    /// Optional. Video height
    pub height: Option<i64>,
    /// Optional. Video duration
    pub duration: Option<i64>,
    /// Optional. Pass True, if the uploaded video is suitable for streaming
    pub supports_streaming: Option<bool>,
}

