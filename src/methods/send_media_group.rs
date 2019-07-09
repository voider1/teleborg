use super::Method;
use crate::types::{Message, MediaGroup};
use serde::Serialize;
use typed_builder::TypedBuilder;
use crate::{error::Result, methods::read_file};
use reqwest::r#async::{
    multipart::{Form, Part},
    RequestBuilder,
};
use std::path::Path;

/// Use this method to send a group of photos or videos as an album. On success, an array of the sent Messages is returned.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SendMediaGroup {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// A JSON-serialized array describing photos and videos to be sent, must include 2–10 items
    pub media: Vec<MediaGroup>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the messages silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the messages are a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
}


impl Method for SendMediaGroup {
    type Response = Vec<Message>;
    const PATH: &'static str = $path;

    fn build(mut self, builder: RequestBuilder) -> Result<RequestBuilder> {

        let form = Form::new();
        self.media.iter().for_each(|m| {
            match m {
                Photo => {
                    let file_path = self.file.take().unwrap();
                    let buffer = read_file(&file_path)?;

                    let path = Path::new(&file_path);
                    let name = path.file_name().unwrap().to_str().unwrap();
                    let part = Part::bytes(buffer).file_name(String::from(name));
                    form.part("media", part);
                },
                Video => {
                
                }
            };
        });

        Ok(builder.query(&self).multipart(form))
    }
}
