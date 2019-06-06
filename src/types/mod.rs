pub use self::{
    animation::Animation, audio::Audio, callback_query::CallbackQuery, chat::Chat,
    chat_action::ChatAction, chat_member::ChatMember, chat_photo::ChatPhoto,
    chosen_inline_result::ChosenInlineResult, contact::Contact, document::Document,
    encrypted_credentials::EncryptedCredentials,
    encrypted_passport_element::EncryptedPassportElement, file::File, force_reply::ForceReply,
    game::Game, game_high_score::GameHighScore, inline_keyboard_button::InlineKeyboardButton,
    inline_query::InlineQuery, inline_query_result::InlineQueryResult,
    input_media_animation::InputMediaAnimation, input_media_audio::InputMediaAudio,
    input_media_document::InputMediaDocument, input_media_photo::InputMediaPhoto,
    input_media_video::InputMediaVideo, input_message_content::InputMessageContent,
    invoice::Invoice, keyboard_button::KeyboardButton, labeled_price::LabeledPrice,
    location::Location, login_url::LoginUrl, mask_position::MaskPosition, message::Message,
    message_entity::MessageEntity, order_info::OrderInfo, parse_mode::ParseMode,
    passport_data::PassportData, passport_element_error_data_field::PassportElementErrorDataField,
    passport_element_error_file::PassportElementErrorFile,
    passport_element_error_files::PassportElementErrorFiles,
    passport_element_error_front_side::PassportElementErrorFrontSide,
    passport_element_error_reverse_side::PassportElementErrorReverseSide,
    passport_element_error_selfie::PassportElementErrorSelfie,
    passport_element_error_translation_file::PassportElementErrorTranslationFile,
    passport_element_error_translation_files::PassportElementErrorTranslationFiles,
    passport_element_error_unspecified::PassportElementErrorUnspecified,
    passport_file::PassportFile, photo_size::PhotoSize, poll::Poll, poll_option::PollOption,
    pre_checkout_query::PreCheckoutQuery, reply_keyboard_remove::ReplyKeyboardRemove,
    reply_markup::ReplyMarkup, response_parameters::ResponseParameters,
    shipping_address::ShippingAddress, shipping_option::ShippingOption,
    shipping_query::ShippingQuery, sticker::Sticker, sticker_set::StickerSet,
    successful_payment::SuccessfulPayment, update::Update, user::User,
    user_profile_photos::UserProfilePhotos, venue::Venue, video::Video, video_note::VideoNote,
    voice::Voice, webhook_info::WebhookInfo,
};

mod animation;
mod audio;
mod callback_query;
mod chat;
mod chat_action;
mod chat_member;
mod chat_photo;
mod chosen_inline_result;
mod contact;
mod document;
mod encrypted_credentials;
mod encrypted_passport_element;
mod file;
mod force_reply;
mod game;
mod game_high_score;
mod inline_keyboard_button;
mod inline_query;
mod inline_query_result;
mod input_media_animation;
mod input_media_audio;
mod input_media_document;
mod input_media_photo;
mod input_media_video;
mod input_message_content;
mod invoice;
mod keyboard_button;
mod labeled_price;
mod location;
mod login_url;
mod mask_position;
mod message;
mod message_entity;
mod order_info;
mod parse_mode;
mod passport_data;
mod passport_element_error_data_field;
mod passport_element_error_file;
mod passport_element_error_files;
mod passport_element_error_front_side;
mod passport_element_error_reverse_side;
mod passport_element_error_selfie;
mod passport_element_error_translation_file;
mod passport_element_error_translation_files;
mod passport_element_error_unspecified;
mod passport_file;
mod photo_size;
mod poll;
mod poll_option;
mod pre_checkout_query;
mod reply_keyboard_remove;
mod reply_markup;
mod response_parameters;
mod shipping_address;
mod shipping_option;
mod shipping_query;
mod sticker;
mod sticker_set;
mod successful_payment;
mod update;
mod user;
mod user_profile_photos;
mod venue;
mod video;
mod video_note;
mod voice;
mod webhook_info;
