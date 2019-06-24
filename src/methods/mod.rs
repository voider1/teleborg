pub use self::{
get_updates::GetUpdates, set_webhook::SetWebhook, send_message::SendMessage, forward_message::ForwardMessage, send_photo::SendPhoto, send_audio::SendAudio, send_document::SendDocument, send_video::SendVideo, send_animation::SendAnimation, send_voice::SendVoice, send_video_note::SendVideoNote, send_location::SendLocation, edit_message_live_location::EditMessageLiveLocation, stop_message_live_location::StopMessageLiveLocation, send_venue::SendVenue, send_contact::SendContact, send_poll::SendPoll, send_chat_action::SendChatAction, get_user_profile_photos::GetUserProfilePhotos, get_file::GetFile, kick_chat_member::KickChatMember, unban_chat_member::UnbanChatMember, restrict_chat_member::RestrictChatMember, promote_chat_member::PromoteChatMember, export_chat_invite_link::ExportChatInviteLink, set_chat_photo::SetChatPhoto, delete_chat_photo::DeleteChatPhoto, set_chat_title::SetChatTitle, set_chat_description::SetChatDescription, pin_chat_message::PinChatMessage, unpin_chat_message::UnpinChatMessage, leave_chat::LeaveChat, get_chat::GetChat, get_chat_administrators::GetChatAdministrators, get_chat_members_count::GetChatMembersCount, get_chat_member::GetChatMember, set_chat_sticker_set::SetChatStickerSet, delete_chat_sticker_set::DeleteChatStickerSet, answer_callback_query::AnswerCallbackQuery, edit_message_text::EditMessageText, edit_message_caption::EditMessageCaption, edit_message_reply_markup::EditMessageReplyMarkup, stop_poll::StopPoll, delete_message::DeleteMessage, send_sticker::SendSticker, get_sticker_set::GetStickerSet, upload_sticker_file::UploadStickerFile, create_new_sticker_set::CreateNewStickerSet, add_sticker_to_set::AddStickerToSet, set_sticker_position_in_set::SetStickerPositionInSet, delete_sticker_from_set::DeleteStickerFromSet, answer_inline_query::AnswerInlineQuery, send_invoice::SendInvoice, answer_shipping_query::AnswerShippingQuery, answer_pre_checkout_query::AnswerPreCheckoutQuery, set_passport_data_errors::SetPassportDataErrors, send_game::SendGame, set_game_score::SetGameScore, get_game_high_scores::GetGameHighScores
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

mod get_updates;
mod set_webhook;
mod send_message;
mod forward_message;
mod send_photo;
mod send_audio;
mod send_document;
mod send_video;
mod send_animation;
mod send_voice;
mod send_video_note;
mod send_location;
mod edit_message_live_location;
mod stop_message_live_location;
mod send_venue;
mod send_contact;
mod send_poll;
mod send_chat_action;
mod get_user_profile_photos;
mod get_file;
mod kick_chat_member;
mod unban_chat_member;
mod restrict_chat_member;
mod promote_chat_member;
mod export_chat_invite_link;
mod set_chat_photo;
mod delete_chat_photo;
mod set_chat_title;
mod set_chat_description;
mod pin_chat_message;
mod unpin_chat_message;
mod leave_chat;
mod get_chat;
mod get_chat_administrators;
mod get_chat_members_count;
mod get_chat_member;
mod set_chat_sticker_set;
mod delete_chat_sticker_set;
mod answer_callback_query;
mod edit_message_text;
mod edit_message_caption;
mod edit_message_reply_markup;
mod stop_poll;
mod delete_message;
mod send_sticker;
mod get_sticker_set;
mod upload_sticker_file;
mod create_new_sticker_set;
mod add_sticker_to_set;
mod set_sticker_position_in_set;
mod delete_sticker_from_set;
mod answer_inline_query;
mod send_invoice;
mod answer_shipping_query;
mod answer_pre_checkout_query;
mod set_passport_data_errors;
mod send_game;
mod set_game_score;
mod get_game_high_scores;

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
