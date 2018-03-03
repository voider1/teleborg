pub use self::send_message::{SendMessage, SendMessageBuilder};
pub use self::forward_message::{ForwardMessage, ForwardMessageBuilder};
pub use self::send_contact::{SendContact, SendContactBuilder};
pub use self::send_chat_action::{SendChatAction, SendChatActionBuilder};
pub use self::get_user_profile_photos::{GetUserProfilePhotos, GetUserProfilePhotosBuilder};
pub use self::get_file::{GetFile, GetFileBuilder};
pub use self::kick_chat_member::{KickChatMember, KickChatMemberBuilder};
pub use self::unban_chat_member::{UnbanChatMember, UnbanChatMemberBuilder};
pub use self::export_chat_invite_link::{ExportChatInviteLink, ExportChatInviteLinkBuilder};

use serde::Serialize;
use serde::de::DeserializeOwned;

use error::Result;
use Bot;

macro_rules! impl_builder {
    ($struct:ident, $struct_builder:ident) => {
        impl $struct {
            pub fn builder() -> $struct_builder {
                $struct_builder::default()
            }
        }
    }
}

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

pub trait Method: Serialize + Sized {
    type Response: DeserializeOwned;
    const PATH: &'static str;

    fn call(&self, bot: &Bot) -> Result<Self::Response> {
        bot.call(Self::PATH, self)
    }
}
