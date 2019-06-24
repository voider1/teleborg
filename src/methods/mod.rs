pub use self::{
    add_sticker_to_set::AddStickerToSet, answer_callback_query::AnswerCallbackQuery,
    answer_inline_query::AnswerInlineQuery, answer_pre_checkout_query::AnswerPreCheckoutQuery,
    answer_shipping_query::AnswerShippingQuery, create_new_sticker_set::CreateNewStickerSet,
    delete_chat_photo::DeleteChatPhoto, delete_chat_sticker_set::DeleteChatStickerSet,
    delete_message::DeleteMessage, delete_sticker_from_set::DeleteStickerFromSet,
    edit_message_caption::EditMessageCaption, edit_message_live_location::EditMessageLiveLocation,
    edit_message_reply_markup::EditMessageReplyMarkup, edit_message_text::EditMessageText,
    export_chat_invite_link::ExportChatInviteLink, forward_message::ForwardMessage,
    get_chat::GetChat, get_chat_administrators::GetChatAdministrators,
    get_chat_member::GetChatMember, get_chat_members_count::GetChatMembersCount, get_file::GetFile,
    get_game_high_scores::GetGameHighScores, get_sticker_set::GetStickerSet,
    get_updates::GetUpdates, get_user_profile_photos::GetUserProfilePhotos,
    kick_chat_member::KickChatMember, leave_chat::LeaveChat, pin_chat_message::PinChatMessage,
    promote_chat_member::PromoteChatMember, restrict_chat_member::RestrictChatMember,
    send_animation::SendAnimation, send_audio::SendAudio, send_chat_action::SendChatAction,
    send_contact::SendContact, send_document::SendDocument, send_game::SendGame,
    send_invoice::SendInvoice, send_location::SendLocation, send_message::SendMessage,
    send_photo::SendPhoto, send_poll::SendPoll, send_sticker::SendSticker, send_venue::SendVenue,
    send_video::SendVideo, send_video_note::SendVideoNote, send_voice::SendVoice,
    set_chat_description::SetChatDescription, set_chat_photo::SetChatPhoto,
    set_chat_sticker_set::SetChatStickerSet, set_chat_title::SetChatTitle,
    set_game_score::SetGameScore, set_passport_data_errors::SetPassportDataErrors,
    set_sticker_position_in_set::SetStickerPositionInSet, set_webhook::SetWebhook,
    stop_message_live_location::StopMessageLiveLocation, stop_poll::StopPoll,
    unban_chat_member::UnbanChatMember, unpin_chat_message::UnpinChatMessage,
    upload_sticker_file::UploadStickerFile,
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

mod add_sticker_to_set;
mod answer_callback_query;
mod answer_inline_query;
mod answer_pre_checkout_query;
mod answer_shipping_query;
mod create_new_sticker_set;
mod delete_chat_photo;
mod delete_chat_sticker_set;
mod delete_message;
mod delete_sticker_from_set;
mod edit_message_caption;
mod edit_message_live_location;
mod edit_message_reply_markup;
mod edit_message_text;
mod export_chat_invite_link;
mod forward_message;
mod get_chat;
mod get_chat_administrators;
mod get_chat_member;
mod get_chat_members_count;
mod get_file;
mod get_game_high_scores;
mod get_sticker_set;
mod get_updates;
mod get_user_profile_photos;
mod kick_chat_member;
mod leave_chat;
mod pin_chat_message;
mod promote_chat_member;
mod restrict_chat_member;
mod send_animation;
mod send_audio;
mod send_chat_action;
mod send_contact;
mod send_document;
mod send_game;
mod send_invoice;
mod send_location;
mod send_message;
mod send_photo;
mod send_poll;
mod send_sticker;
mod send_venue;
mod send_video;
mod send_video_note;
mod send_voice;
mod set_chat_description;
mod set_chat_photo;
mod set_chat_sticker_set;
mod set_chat_title;
mod set_game_score;
mod set_passport_data_errors;
mod set_sticker_position_in_set;
mod set_webhook;
mod stop_message_live_location;
mod stop_poll;
mod unban_chat_member;
mod unpin_chat_message;
mod upload_sticker_file;

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
