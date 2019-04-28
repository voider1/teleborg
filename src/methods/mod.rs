pub use self::{
    edit_message_text::EditMessageText, export_chat_invite_link::ExportChatInviteLink,
    forward_message::ForwardMessage, get_file::GetFile, get_updates::GetUpdates,
    get_user_profile_photos::GetUserProfilePhotos, kick_chat_member::KickChatMember,
    send_animation::SendAnimation, send_audio::SendAudio, send_chat_action::SendChatAction,
    send_contact::SendContact, send_document::SendDocument, send_location::SendLocation,
    send_message::SendMessage, send_photo::SendPhoto, send_venue::SendVenue, send_video::SendVideo,
    send_video_note::SendVideoNote, send_voice::SendVoice, unban_chat_member::UnbanChatMember,
};
use crate::error::{Error, Result};
use reqwest::r#async::RequestBuilder;
use serde::{de::DeserializeOwned, Serialize};
use std::{fs::File, io::Read};

macro_rules! impl_method {
    ($struct:ident, $response:ty, $path:expr) => {
        impl Method for $struct {
            type Response = $response;
            const PATH: &'static str = $path;
        }
    };
}

macro_rules! impl_method_multipart {
    ($struct:ident, $response:ident, $path:expr, $filefield:expr) => {
        use crate::{error::Result, methods::read_file};
        use reqwest::r#async::{
            multipart::{Form, Part},
            RequestBuilder,
        };
        use std::path::Path;

        impl Method for $struct {
            type Response = $response;
            const PATH: &'static str = $path;

            fn build(mut self, builder: RequestBuilder) -> Result<RequestBuilder> {
                if self.file.is_none() {
                    return Ok(builder.json(&self));
                }

                let file_path = self.file.take().unwrap();
                let buffer = read_file(&file_path)?;

                let path = Path::new(&file_path);
                let name = path.file_name().unwrap().to_str().unwrap();
                let part = Part::bytes(buffer).file_name(String::from(name));
                let form = Form::new().part($filefield, part);

                Ok(builder.query(&self).multipart(form))
            }
        }
    };
}

macro_rules! impl_method_multipart_thumb {
    ($struct:ident, $response:ident, $path:expr, $filefield:expr) => {
        use crate::{error::Result, methods::read_file};
        use reqwest::r#async::{
            multipart::{Form, Part},
            RequestBuilder,
        };
        use std::path::Path;

        impl Method for $struct {
            type Response = $response;
            const PATH: &'static str = $path;

            fn build(mut self, builder: RequestBuilder) -> Result<RequestBuilder> {
                if self.file.is_none() && self.thumb_file.is_none() {
                    return Ok(builder.json(&self));
                }
                let mut form = Form::new();

                if let Some(file_path) = self.file.take() {
                    let buffer = read_file(&file_path)?;
                    let name = Path::new(&file_path).file_name().unwrap().to_str().unwrap();
                    let part = Part::bytes(buffer).file_name(String::from(name));
                    form = form.part($filefield, part);
                }

                if let Some(thumb_file_path) = self.thumb_file.take() {
                    let thumb_buffer = read_file(&thumb_file_path)?;
                    let thumb_name = Path::new(&thumb_file_path)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap();
                    let thumb_part = Part::bytes(thumb_buffer).file_name(String::from(thumb_name));
                    form = form.part("thumb", thumb_part);
                }

                Ok(builder.query(&self).multipart(form))
            }
        }
    };
}

/// Reads file from path given and returns a byte vector with max size of 50 megabytes
fn read_file(file_path: &str) -> Result<Vec<u8>> {
    // 50 MB capacity because this is the maximum the bot API accepts
    let mut buffer = Vec::with_capacity(50_000_000);

    let mut file = File::open(file_path)
        .map_err(|e| Error::MultiPartBuilderError(format!("File couldn't open: {}", e)))?;

    file.read_to_end(&mut buffer)
        .map_err(|e| Error::MultiPartBuilderError(format!("File couldn't be read: {}", e)))?;
    Ok(buffer)
}

mod edit_message_text;
mod export_chat_invite_link;
mod forward_message;
mod get_file;
mod get_updates;
mod get_user_profile_photos;
mod kick_chat_member;
mod send_animation;
mod send_audio;
mod send_chat_action;
mod send_contact;
mod send_document;
mod send_location;
mod send_message;
mod send_photo;
mod send_venue;
mod send_video;
mod send_video_note;
mod send_voice;
mod unban_chat_member;

/// This trait gets implemented for every method-builder and makes sure that they all have an
/// associated path and that we know what we're expecting to receive from the server.
pub trait Method: Serialize + Sized {
    /// Expected response from the server.
    type Response: DeserializeOwned + Send;

    /// Associated path for the method we implement this on.
    const PATH: &'static str;

    /// Method for building the request.
    fn build(self, builder: RequestBuilder) -> Result<RequestBuilder> {
        Ok(builder.json(&self))
    }
}
