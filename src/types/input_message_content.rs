use crate::types::ParseMode;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    InputTextMessageContent {
        /// Text of the message to be sent, 1-4096 characters
        message_text: String,
        /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in your bot's message.
        parse_mode: Option<ParseMode>,
        /// Optional. Disables link previews for links in the sent message
        disable_web_page_preview: Option<bool>,
    },
    InputLocationMessageContent {
        /// Latitude of the location in degrees
        latitude: f64,
        /// Longitude of the location in degrees
        longitude: f64,
        /// Optional. Period in seconds for which the location can be updated, should be between 60 and 86400.
        live_period: Option<i64>,
    },
    InputVenueMessageContent {
        /// Latitude of the venue in degrees
        latitude: f64,
        /// Longitude of the venue in degrees
        longitude: f64,
        /// Name of the venue
        title: String,
        /// Address of the venue
        address: String,
        /// Optional. Foursquare identifier of the venue, if known
        foursquare_id: Option<String>,
        /// Optional. Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
        foursquare_type: Option<String>,
    },
    InputContactMessageContent {
        /// Contact's phone number
        phone_number: String,
        /// Contact's first name
        first_name: String,
        /// Optional. Contact's last name
        last_name: Option<String>,
        /// Optional. Additional data about the contact in the form of a vCard, 0-2048 bytes
        vcard: Option<String>,
    },
}
