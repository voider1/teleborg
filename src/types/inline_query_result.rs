use crate::types::{InputMessageContent, ParseMode, ReplyMarkup};
use serde::{Deserialize, Serialize};

/// This object represents one result of an inline query. Telegram clients currently support results of the following 20 types.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InlineQueryResult {
    /// Represents a link to an article or web page.
    Article {
        /// Type of the result, must be article
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 Bytes
        id: String,
        /// Title of the result
        title: String,
        /// Content of the message to be sent
        input_message_content: InputMessageContent,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. URL of the result
        url: Option<String>,
        /// Optional. Pass True, if you don't want the URL to be shown in the message
        hide_url: Option<bool>,
        /// Optional. Short description of the result
        description: Option<String>,
        /// Optional. Url of the thumbnail for the result
        thumb_url: Option<String>,
        /// Optional. Thumbnail width
        thumb_width: Option<i64>,
        /// Optional. Thumbnail height
        thumb_height: Option<i64>,
    },
    /// Represents a link to a photo. By default, this photo will be sent by the user with optional
    /// caption. Alternatively, you can use input_message_content to send a message with the
    /// specified content instead of the photo.
    Photo {
        /// Type of the result, must be photo
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid URL of the photo. Photo must be in jpeg format. Photo size must not exceed 5MB
        photo_url: String,
        /// URL of the thumbnail for the photo
        thumb_url: String,
        /// Optional. Width of the photo
        photo_width: Option<i64>,
        /// Optional. Height of the photo
        photo_height: Option<i64>,
        /// Optional. Title for the result
        title: Option<String>,
        /// Optional. Short description of the result
        description: Option<String>,
        /// Optional. Caption of the photo to be sent, 0-1024 characters
        caption: Option<String>,
        /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
        parse_mode: Option<ParseMode>,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. Content of the message to be sent instead of the photo
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to an animated GIF file. By default, this animated GIF file will be sent
    /// by the user with optional caption. Alternatively, you can use input_message_content to send
    /// a message with the specified content instead of the animation.
    Gif {
        /// Type of the result, must be gif
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid URL for the GIF file. File size must not exceed 1MB
        gif_url: String,
        /// Optional. Width of the GIF
        gif_width: Option<i64>,
        /// Optional. Height of the GIF
        gif_height: Option<i64>,
        /// Optional. Duration of the GIF
        gif_duration: Option<i64>,
        /// URL of the static thumbnail for the result (jpeg or gif)
        thumb_url: String,
        /// Optional. Title for the result
        title: Option<String>,
        /// Optional. Caption of the GIF file to be sent, 0-1024 characters
        caption: Option<String>,
        /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
        parse_mode: Option<ParseMode>,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. Content of the message to be sent instead of the GIF animation
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default,
    /// this animated MPEG-4 file will be sent by the user with optional caption. Alternatively,
    /// you can use input_message_content to send a message with the specified content instead of
    /// the animation.
    Mpeg4Gif {
        /// Type of the result, must be mpeg4_gif
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid URL for the MP4 file. File size must not exceed 1MB
        mpeg4_url: String,
        /// Optional. Video width
        mpeg4_width: Option<i64>,
        /// Optional. Video height
        mpeg4_height: Option<i64>,
        /// Optional. Video duration
        mpeg4_duration: Option<i64>,
        /// URL of the static thumbnail (jpeg or gif) for the result
        thumb_url: String,
        /// Optional. Title for the result
        title: Option<String>,
        /// Optional. Caption of the MPEG-4 file to be sent, 0-1024 characters
        caption: Option<String>,
        /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
        parse_mode: Option<ParseMode>,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. Content of the message to be sent instead of the video animation
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to a page containing an embedded video player or a video file. By
    /// default, this video file will be sent by the user with an optional caption. Alternatively,
    /// you can use input_message_content to send a message with the specified content instead of
    /// the video.
    Video {
        /// Type of the result, must be video
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid URL for the embedded video player or video file
        video_url: String,
        /// Mime type of the content of video url, “text/html” or “video/mp4”
        mime_type: String,
        /// URL of the thumbnail (jpeg only) for the video
        thumb_url: String,
        /// Title for the result
        title: String,
        /// Optional. Caption of the video to be sent, 0-1024 characters
        caption: Option<String>,
        /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
        parse_mode: Option<ParseMode>,
        /// Optional. Video width
        video_width: Option<i64>,
        /// Optional. Video height
        video_height: Option<i64>,
        /// Optional. Video duration in seconds
        video_duration: Option<i64>,
        /// Optional. Short description of the result
        description: Option<String>,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. Content of the message to be sent instead of the video. This field is required if Video is used to send an HTML-page as a result (e.g., a YouTube video).
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to an mp3 audio file. By default, this audio file will be sent by the
    /// user. Alternatively, you can use input_message_content to send a message with the specified
    /// content instead of the audio.
    Audio {
        /// Type of the result, must be audio
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid URL for the audio file
        audio_url: String,
        /// Title
        title: String,
        /// Optional. Caption, 0-1024 characters
        caption: Option<String>,
        /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
        parse_mode: Option<ParseMode>,
        /// Optional. Performer
        performer: Option<String>,
        /// Optional. Audio duration in seconds
        audio_duration: Option<i64>,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. Content of the message to be sent instead of the audio
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to a voice recording in an .ogg container encoded with OPUS. By default,
    /// this voice recording will be sent by the user. Alternatively, you can use
    /// input_message_content to send a message with the specified content instead of the the voice
    /// message.
    Voice {
        /// Type of the result, must be voice
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid URL for the voice recording
        voice_url: String,
        /// Recording title
        title: String,
        /// Optional. Caption, 0-1024 characters
        caption: Option<String>,
        /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
        parse_mode: Option<ParseMode>,
        /// Optional. Recording duration in seconds
        voice_duration: Option<i64>,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. Content of the message to be sent instead of the voice recording
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to a file. By default, this file will be sent by the user with an
    /// optional caption. Alternatively, you can use input_message_content to send a message with
    /// the specified content instead of the file. Currently, only .PDF and .ZIP files can be sent
    /// using this method.
    Document {
        /// Type of the result, must be document
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// Title for the result
        title: String,
        /// Optional. Caption of the document to be sent, 0-1024 characters
        caption: Option<String>,
        /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
        parse_mode: Option<ParseMode>,
        /// A valid URL for the file
        document_url: String,
        /// Mime type of the content of the file, either “application/pdf” or “application/zip”
        mime_type: String,
        /// Optional. Short description of the result
        description: Option<String>,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. Content of the message to be sent instead of the file
        input_message_content: Option<InputMessageContent>,
        /// Optional. URL of the thumbnail (jpeg only) for the file
        thumb_url: Option<String>,
        /// Optional. Thumbnail width
        thumb_width: Option<i64>,
        /// Optional. Thumbnail height
        thumb_height: Option<i64>,
    },
    /// Represents a location on a map. By default, the location will be sent by the user.
    /// Alternatively, you can use input_message_content to send a message with the specified
    /// content instead of the location.
    Location {
        /// Type of the result, must be location
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 Bytes
        id: String,
        /// Location latitude in degrees
        latitude: f64,
        /// Location longitude in degrees
        longitude: f64,
        /// Location title
        title: String,
        /// Optional. Period in seconds for which the location can be updated, should be between 60 and 86400.
        live_period: Option<i64>,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. Content of the message to be sent instead of the location
        input_message_content: Option<InputMessageContent>,
        /// Optional. Url of the thumbnail for the result
        thumb_url: Option<String>,
        /// Optional. Thumbnail width
        thumb_width: Option<i64>,
        /// Optional. Thumbnail height
        thumb_height: Option<i64>,
    },
    /// Represents a venue. By default, the venue will be sent by the user. Alternatively, you can
    /// use input_message_content to send a message with the specified content instead of the
    /// venue.
    Venue {
        /// Type of the result, must be venue
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 Bytes
        id: String,
        /// Latitude of the venue location in degrees
        latitude: f64,
        /// Longitude of the venue location in degrees
        longitude: f64,
        /// Title of the venue
        title: String,
        /// Address of the venue
        address: String,
        /// Optional. Foursquare identifier of the venue if known
        foursquare_id: Option<String>,
        /// Optional. Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
        foursquare_type: Option<String>,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. Content of the message to be sent instead of the venue
        input_message_content: Option<InputMessageContent>,
        /// Optional. Url of the thumbnail for the result
        thumb_url: Option<String>,
        /// Optional. Thumbnail width
        thumb_width: Option<i64>,
        /// Optional. Thumbnail height
        thumb_height: Option<i64>,
    },
    /// Represents a contact with a phone number. By default, this contact will be sent by the
    /// user. Alternatively, you can use input_message_content to send a message with the specified
    /// content instead of the contact.
    Contact {
        /// Type of the result, must be contact
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 Bytes
        id: String,
        /// Contact's phone number
        phone_number: String,
        /// Contact's first name
        first_name: String,
        /// Optional. Contact's last name
        last_name: Option<String>,
        /// Optional. Additional data about the contact in the form of a vCard, 0-2048 bytes
        vcard: Option<String>,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. Content of the message to be sent instead of the contact
        input_message_content: Option<InputMessageContent>,
        /// Optional. Url of the thumbnail for the result
        thumb_url: Option<String>,
        /// Optional. Thumbnail width
        thumb_width: Option<i64>,
        /// Optional. Thumbnail height
        thumb_height: Option<i64>,
    },
    /// Represents a Game.
    Game {
        /// Type of the result, must be game
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// Short name of the game
        game_short_name: String,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
    },
    /// Represents a link to a photo stored on the Telegram servers. By default,
    /// this photo will be sent by the user with an optional caption.
    /// Alternatively, you can use input_message_content to send a message with
    /// the specified content instead of the photo.
    CachedPhoto {
        /// Type of the result, must be photo
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid file identifier of the photo
        photo_file_id: String,
        /// Optional. Title for the result
        title: Option<String>,
        /// Optional. Short description of the result
        description: Option<String>,
        /// Optional. Caption of the photo to be sent, 0-1024 characters
        caption: Option<String>,
        /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
        parse_mode: Option<ParseMode>,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. Content of the message to be sent instead of the photo
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to an animated GIF file stored on the Telegram servers. By default, this
    /// animated GIF file will be sent by the user with an optional caption. Alternatively, you can
    /// use input_message_content to send a message with specified content instead of the
    /// animation.
    CachedGif {
        /// Type of the result, must be gif
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid file identifier for the GIF file
        gif_file_id: String,
        /// Optional. Title for the result
        title: Option<String>,
        /// Optional. Caption of the GIF file to be sent, 0-1024 characters
        caption: Option<String>,
        /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
        parse_mode: Option<ParseMode>,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. Content of the message to be sent instead of the GIF animation
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers.
    /// By default, this animated MPEG-4 file will be sent by the user with an optional caption. Alternatively,
    /// you can use input_message_content to send a message with the specified content instead of the animation.
    CachedMpeg4Gif {
        /// Type of the result, must be mpeg4_gif
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid file identifier for the MP4 file
        mpeg4_file_id: String,
        /// Optional. Title for the result
        title: Option<String>,
        /// Optional. Caption of the MPEG-4 file to be sent, 0-1024 characters
        caption: Option<String>,
        /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
        parse_mode: Option<ParseMode>,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. Content of the message to be sent instead of the video animation
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to a sticker stored on the Telegram servers. By default, this sticker
    /// will be sent by the user. Alternatively, you can use input_message_content to send a
    /// message with the specified content instead of the sticker.
    CachedSticker {
        /// Type of the result, must be sticker
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid file identifier of the sticker
        sticker_file_id: String,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. Content of the message to be sent instead of the sticker
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to a file stored on the Telegram servers. By default, this file will be
    /// sent by the user with an optional caption. Alternatively, you can use input_message_content
    /// to send a message with the specified content instead of the file.
    CachedDocument {
        /// Type of the result, must be document
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// Title for the result
        title: String,
        /// A valid file identifier for the file
        document_file_id: String,
        /// Optional. Short description of the result
        description: Option<String>,
        /// Optional. Caption of the document to be sent, 0-1024 characters
        caption: Option<String>,
        /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
        parse_mode: Option<ParseMode>,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. Content of the message to be sent instead of the file
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to a video file stored on the Telegram servers. By default, this video
    /// file will be sent by the user with an optional caption. Alternatively, you can use
    /// input_message_content to send a message with the specified content instead of the video.
    CachedVideo {
        /// Type of the result, must be video
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid file identifier for the video file
        video_file_id: String,
        /// Title for the result
        title: String,
        /// Optional. Short description of the result
        description: Option<String>,
        /// Optional. Caption of the video to be sent, 0-1024 characters
        caption: Option<String>,
        /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
        parse_mode: Option<ParseMode>,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. Content of the message to be sent instead of the video
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to a voice message stored on the Telegram servers. By default, this voice
    /// message will be sent by the user. Alternatively, you can use input_message_content to send
    /// a message with the specified content instead of the voice message.
    CachedVoice {
        /// Type of the result, must be voice
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid file identifier for the voice message
        voice_file_id: String,
        /// Voice message title
        title: String,
        /// Optional. Caption, 0-1024 characters
        caption: Option<String>,
        /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
        parse_mode: Option<ParseMode>,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. Content of the message to be sent instead of the voice message
        input_message_content: Option<InputMessageContent>,
    },
    /// Represents a link to an mp3 audio file stored on the Telegram servers. By default, this
    /// audio file will be sent by the user. Alternatively, you can use input_message_content to
    /// send a message with the specified content instead of the audio.
    CachedAudio {
        /// Type of the result, must be audio
        #[serde(rename = "type")]
        kind: String,
        /// Unique identifier for this result, 1-64 bytes
        id: String,
        /// A valid file identifier for the audio file
        audio_file_id: String,
        /// Optional. Caption, 0-1024 characters
        caption: Option<String>,
        /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
        parse_mode: Option<ParseMode>,
        /// Optional. Inline keyboard attached to the message
        reply_markup: Option<ReplyMarkup>,
        /// Optional. Content of the message to be sent instead of the audio
        input_message_content: Option<InputMessageContent>,
    },
}
