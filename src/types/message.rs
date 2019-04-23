use super::{
    Audio, Chat, Contact, Document, Game, Invoice, Location, MessageEntity, PhotoSize, Sticker,
    SuccessfulPayment, User, Venue, Video, VideoNote, Voice,
};
use serde::Deserialize;

/// This struct represents a message.
#[derive(Clone, Deserialize, Debug)]
pub struct Message {
    /// Unique message identifier inside this chat.
    pub message_id: i32,
    /// Sender, empty for messages sent to channels.
    pub from: Option<User>,
    /// Date the message was sent (Unix time).
    pub date: i32,
    /// Chat the message belongs to.
    pub chat: Chat,
    /// For forwarded messages, sender of the original message.
    pub forward_from: Option<User>,
    /// For messages forwarded from channels, infomration about the original channel.
    pub forward_from_chat: Option<Chat>,
    /// For messages forwarded from channels, identifier of the original message in the channel.
    pub forward_from_message_id: Option<i32>,
    /// For messages forwarded from channels, signature of the post author if present.
    pub forward_signature: Option<String>,
    /// For forwarded messages, date of the original message was sent (Unix time).
    pub forward_date: Option<i32>,
    /// For replies, the original message. Note that the `Message` struct in this field will not
    /// contain further `reply_to_message` fields, even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Date the message was last edited (Unix time).
    pub edit_date: Option<i32>,
    /// The unique identifier of a media message group this message belongs to.
    pub media_group_id: Option<String>,
    /// Signature of the post author for messages in channels.
    pub author_signature: Option<String>,
    /// For text messages, the actual UTF-8 text of the message, 0-4096 characters.
    pub text: Option<String>,
    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in
    /// the text.
    pub entities: Option<Vec<MessageEntity>>,
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that
    /// appear in the caption.
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Message is an audio file, information about the file.
    pub audio: Option<Audio>,
    /// Message is a general file, information about the file.
    pub document: Option<Document>,
    /// Message is a game, information about the game.
    pub game: Option<Game>,
    /// Message is a photo, available sizes of the photo.
    pub photo: Option<Vec<PhotoSize>>,
    /// Message is a sticker, information about the sticker.
    pub sticker: Option<Sticker>,
    /// Message is a video, information about the video.
    pub video: Option<Video>,
    /// Message is a voice message, information about the file.
    pub voice: Option<Voice>,
    /// Message is a [video note](https://telegram.org/blog/video-messages-and-telescope),
    /// information about the video message.
    pub video_note: Option<VideoNote>,
    /// Caption for the audio, document, photo, video or voice, 0-200 characters.
    pub caption: Option<String>,
    /// Message is a shared contact, information about the contact.
    pub contact: Option<Contact>,
    /// Message is a shared location, information about the location.
    pub location: Option<Location>,
    /// Message is a venue, information about the video.
    pub venue: Option<Venue>,
    /// New members that were added to the group or supergroup and information about them (the bot
    /// itself may be one of these members).
    pub new_chat_member: Option<User>,
    /// A member was removed from the group, information about them (this member may be the bot
    /// itself).
    pub left_chat_member: Option<User>,
    /// A chat title was changed to this value.
    pub new_chat_title: Option<String>,
    /// A chat photo was changed to this value.
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    /// Service message: The chat photo was deleted.
    pub delete_chat_photo: Option<bool>,
    /// Service message: The group has been created.
    pub group_chat_created: Option<bool>,
    /// Service message: The supergroup has been created. This field can't be received in a message
    /// coming through updates, because the bot can't be a member of a supergroup when it is
    /// created. It can only be found in `reply_to_message` if someone replies to a very first
    /// message in a directly created supergroup.
    pub supergroup_chat_created: Option<bool>,
    /// Service message: The channel has been created. This field can't be received in a message
    /// coming through updates, because the bot can't be a member of a channel when it is created.
    /// It can only be found in `reply_to_message` if someone replies to a very first message in a
    /// channel.
    pub channel_chat_created: Option<bool>,
    /// The group has been migrated to a supergroup with the specified identifier.
    pub migrate_to_chat_id: Option<i64>,
    /// The supergroup has been migrated from a group with the specified identifier.
    pub migrate_from_chat_id: Option<i64>,
    /// Specified message was pinned. Note that the `Message` instance in this field
    /// will not contain further `reply_to_message` fields even if it is itself a reply.
    pub pinned_message: Option<Box<Message>>,
    /// Message is an invoice for a [payment](https://core.telegram.org/bots/api#payments),
    /// information about the invoice.
    pub invoice: Option<Invoice>,
    /// Message is a service message about a successful payment, information about the payment.
    pub successful_payment: Option<SuccessfulPayment>,
    /// The domain name of the website on which the user has logged in.
    pub connected_website: Option<String>,
}
