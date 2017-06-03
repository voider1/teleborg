pub use self::inline_query_result_article::InlineQueryResultArticle;
pub use self::inline_query_result_photo::InlineQueryResultPhoto;
pub use self::inline_query_result_gif::InlineQueryResultGif;
pub use self::inline_query_result_mpeg4_gif::InlineQueryResultMpeg4Gif;
pub use self::inline_query_result_video::InlineQueryResultVideo;
pub use self::inline_query_result_audio::InlineQueryResultAudio;
pub use self::marker::InlineQueryResult;
pub use self::inline_query_result_type::InlineQueryResultType;

mod inline_query_result_article;
mod inline_query_result_photo;
mod inline_query_result_gif;
mod inline_query_result_mpeg4_gif;
mod inline_query_result_video;
mod inline_query_result_audio;
mod marker;
mod inline_query_result_type;

use serde::{Serialize, Serializer};

impl Serialize for Box<InlineQueryResult> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match self.get_type() {
            InlineQueryResultType::Article => {
                serializer.serialize_some((&**self)
                                              .as_any()
                                              .downcast_ref::<InlineQueryResultArticle>()
                                              .unwrap())
            }
            InlineQueryResultType::Photo => {
                serializer.serialize_some((&**self)
                                              .as_any()
                                              .downcast_ref::<InlineQueryResultPhoto>()
                                              .unwrap())
            }
            InlineQueryResultType::Gif => {
                serializer.serialize_some((&**self)
                                              .as_any()
                                              .downcast_ref::<InlineQueryResultGif>()
                                              .unwrap())
            }
            InlineQueryResultType::Mpeg4Gif => {
                serializer.serialize_some((&**self)
                                              .as_any()
                                              .downcast_ref::<InlineQueryResultMpeg4Gif>()
                                              .unwrap())
            }
            InlineQueryResultType::Video => {
                serializer.serialize_some((&**self)
                                              .as_any()
                                              .downcast_ref::<InlineQueryResultVideo>()
                                              .unwrap())
            }
            InlineQueryResultType::Audio => {
                serializer.serialize_some((&**self)
                                              .as_any()
                                              .downcast_ref::<InlineQueryResultAudio>()
                                              .unwrap())
            }
        }
    }
}
