pub use self::edit_message_text::EditMessageText;
pub use self::export_chat_invite_link::ExportChatInviteLink;
pub use self::forward_message::ForwardMessage;
pub use self::get_file::GetFile;
pub use self::get_user_profile_photos::GetUserProfilePhotos;
pub use self::kick_chat_member::KickChatMember;
pub use self::send_chat_action::SendChatAction;
pub use self::send_contact::SendContact;
pub use self::send_location::SendLocation;
pub use self::send_message::SendMessage;
pub use self::send_photo::SendPhoto;
pub use self::unban_chat_member::UnbanChatMember;

use serde::de::DeserializeOwned;
use serde::Serialize;

use reqwest::r#async::RequestBuilder;

macro_rules! impl_method {
    ($struct:ident, $response:ident, $path:expr) => {
        impl Method for $struct {
            type Response = $response;
            const PATH: &'static str = $path;
        }
    };
}

macro_rules! impl_method_multipart {
    ($struct:ident, $response:ident, $path:expr, $filefield:expr) => {
        use reqwest::r#async::multipart::{Form, Part};
        use reqwest::r#async::RequestBuilder;
        use std::fs::File;
        use std::io::Read;
        use std::path::Path;

        impl Method for $struct {
            type Response = $response;
            const PATH: &'static str = $path;

            fn build(&self, builder: RequestBuilder) -> RequestBuilder {
                // Check if file field was filled.
                if let Some(file_path) = &self.file {
                    // Check if path opens.
                    if let Ok(mut file) = File::open(&file_path) {
                        let mut buffer = Vec::new();
                        // Check if file reads successfully.
                        if let Ok(_) = file.read_to_end(&mut buffer) {
                            let path = Path::new(&file_path);

                            let name = path.file_name().unwrap().to_str().unwrap();

                            let part = Part::bytes(buffer).file_name(String::from(name));

                            let form = Form::new().part($filefield, part);
                            return builder.query(self).multipart(form);
                        }
                    }
                }
                // default output
                builder.json(self)
            }
        }
    };
}

mod edit_message_text;
mod export_chat_invite_link;
mod forward_message;
mod get_file;
mod get_user_profile_photos;
mod kick_chat_member;
mod send_chat_action;
mod send_contact;
mod send_location;
mod send_message;
mod send_photo;
mod unban_chat_member;

/// This trait gets implemented for every method-builder and makes sure that they all have an
/// associated path and that we know what we're expecting to receive from the server.
pub trait Method: Serialize + Sized {
    /// Expected response from the server.
    type Response: DeserializeOwned;

    /// Associated path for the method we implement this on.
    const PATH: &'static str;

    /// Method for building the request.
    fn build(&self, builder: RequestBuilder) -> RequestBuilder {
        builder.json(self)
    }
}
