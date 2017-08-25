pub use self::input_text_message_content::InputTextMessageContent;
pub use self::input_location_message_content::InputLocationMessageContent;
pub use self::input_venue_message_content::InputVenueMessageContent;
pub use self::input_contact_message_content::InputContactMessageContent;
pub use self::marker::InputMessageContent;
pub use self::input_message_content_type::InputMessageContentType;


mod input_text_message_content;
mod input_location_message_content;
mod input_venue_message_content;
mod input_contact_message_content;
mod marker;
mod input_message_content_type;

use serde::{Serialize, Serializer};

impl Serialize for Box<InputMessageContent> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match self.get_type() {
            InputMessageContentType::Text => {
                serializer.serialize_some((&**self)
                                              .as_any()
                                              .downcast_ref::<InputTextMessageContent>()
                                              .unwrap())
            }
            InputMessageContentType::Location => {
                serializer.serialize_some((&**self)
                                              .as_any()
                                              .downcast_ref::<InputLocationMessageContent>()
                                              .unwrap())
            }
            InputMessageContentType::Contact => {
                serializer.serialize_some((&**self)
                                              .as_any()
                                              .downcast_ref::<InputContactMessageContent>()
                                              .unwrap())
            }
            InputMessageContentType::Venue => {
                serializer.serialize_some((&**self)
                                              .as_any()
                                              .downcast_ref::<InputVenueMessageContent>()
                                              .unwrap())
            }
        }
    }
}
