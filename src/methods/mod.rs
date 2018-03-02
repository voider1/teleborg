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
