pub use self::send_message::SendMessage;
pub use self::forward_message::ForwardMessage;
pub use self::send_contact::SendContact;
pub use self::send_chat_action::SendChatAction;
pub use self::get_user_profile_photos::GetUserProfilePhotos;
pub use self::get_file::GetFile;
pub use self::kick_chat_member::KickChatMember;
pub use self::unban_chat_member::UnbanChatMember;
pub use self::export_chat_invite_link::ExportChatInviteLink;

use serde::Serialize;
use serde::de::DeserializeOwned;

use error::Result;
use Bot;

macro_rules! impl_method {
    ($struct:ident, $response:ident, $path:expr) => {
        impl Method for $struct {
            type Response = $response;
            const PATH: &'static str = $path;
        }
    }
}

mod send_message;
mod forward_message;
mod send_contact;
mod send_chat_action;
mod get_user_profile_photos;
mod get_file;
mod kick_chat_member;
mod unban_chat_member;
mod export_chat_invite_link;

/// This trait gets implemented for every method-builder and makes sure that they all have an
/// associated path and that we know what we're expecting to receive from the server.
pub trait Method: Serialize + Sized {
    /// Expected response from the server.
    type Response: DeserializeOwned;
    /// Associated path for the method we implement this on.
    const PATH: &'static str;

    /// Makes a request to the Telegram server with the method you're calling this on.
    fn call(&self, bot: &Bot) -> Result<Self::Response> {
        bot.call(Self::PATH, self)
    }
}
