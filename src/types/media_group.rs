use crate::types::ParseMode;
use serde::{Deserialize, Serialize};

/// This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 4 types.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MediaGroup {
    /// Represents the content of a text message to be sent as the result of an inline query.
    Photo {
        /// Type of the result, must be photo
        #[serde(rename = "type")]
        kind: String,
        /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More info on Sending Files »
        media: String,
        /// Optional. Caption of the photo to be sent, 0-1024 characters
        caption: Option<String>,
        /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
        parse_mode: Option<ParseMode>,
    },
    /// Represents the content of a location message to be sent as the result of an inline query.
    Video {
        /// Type of the result, must be video
        #[serde(rename = "type")]
        kind: String,
        /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More info on Sending Files »
        media: String,
        #[serde(skip_serializing)]
        /// thumb file to send with multipart
        thumb_file: Option<String>,
        /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
        thumb: Option<String>,
        /// Optional. Caption of the video to be sent, 0-1024 characters
        caption: Option<String>,
        /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
        parse_mode: Option<ParseMode>,
        /// Optional. Video width
        width: Option<i64>,
        /// Optional. Video height
        height: Option<i64>,
        /// Optional. Video duration
        duration: Option<i64>,
        /// Optional. Pass True, if the uploaded video is suitable for streaming
        supports_streaming: Option<bool>,
    },
}
