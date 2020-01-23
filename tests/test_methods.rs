#[cfg(test)]
mod tests {
    use reqwest::{r#async::Client as AsyncClient, Client};
    use teleborg::types::{ChatAction, ParseMode};
    use teleborg::{methods::*, spawn, types::Update, Bot, Dispatcher, Future, Updater};
    use tokio::prelude::*;

    use std::{env, sync::Arc};

    fn setup() -> (String, i64) {
        let token = env::var("TELEBORG_TEST_TOKEN")
            .ok()
            .expect("Can't find TELEBORG_TEST_TOKEN env variable");
        let chat_id = env::var("TELEBORG_TEST_CHAT_ID")
            .ok()
            .expect("Can't find TELEBORG_TEST_CHAT_ID env variable")
            .parse::<i64>()
            .unwrap();
        (token, chat_id)
    }

    #[test]
    // Because the get_me gets called when a new bot is made.
    // this is the way to test it.
    fn test_get_me() {
        let (token, _) = setup();
        let bot = Bot::new(&token);
        assert!(bot.is_ok());
    }

    #[test]
    fn test_send_message() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token).unwrap();

        tokio::run(futures::lazy(move || {
            let text = String::from("Test SendMessage");
            let msg = SendMessage::builder().chat_id(chat_id).text(text).build();

            spawn(bot.call(msg).then(|result| {
                assert!(result.is_ok());
                Ok(())
            }))
        }));
    }

    #[test]
    fn test_forward_message() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token.clone()).unwrap();
        let bot2 = bot.clone();

        tokio::run(futures::lazy(move || {
            let text = String::from("Test ForwardMessage");
            let msg = SendMessage::builder().chat_id(chat_id).text(text).build();

            spawn(bot.call(msg).then(move |result| {
                assert!(result.is_ok());
                let msg = ForwardMessage::builder()
                    .chat_id(chat_id)
                    .from_chat_id(chat_id)
                    .message_id(result.unwrap().message_id)
                    .build();
                spawn(bot2.call(msg).then(move |result| {
                    assert!(result.is_ok());
                    Ok(())
                }));
                Ok(())
            }))
        }));
    }

    #[test]
    fn test_send_photo() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token.clone()).unwrap();
        tokio::run(futures::lazy(move || {
            let msg = SendPhoto::builder()
                .chat_id(chat_id)
                .file("test_media/photos/crab.png".to_string())
                .caption("Test SendPhoto".to_string())
                .build();

            spawn(bot.call(msg).then(move |result| {
                assert!(result.is_ok());
                Ok(())
            }))
        }));
    }

    #[test]
    fn test_send_audio() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token.clone()).unwrap();
        tokio::run(futures::lazy(move || {
            let msg = SendAudio::builder()
                .chat_id(chat_id)
                .file("test_media/audio/audio.mp3".to_string())
                .caption("Test SendAudio".to_string())
                .build();

            spawn(bot.call(msg).then(move |result| {
                assert!(result.is_ok());
                Ok(())
            }))
        }));
    }

    #[test]
    fn test_send_document() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token.clone()).unwrap();
        tokio::run(futures::lazy(move || {
            let msg = SendDocument::builder()
                .chat_id(chat_id)
                .file("test_media/document.pdf".to_string())
                .caption("Test SendDocument".to_string())
                .build();

            spawn(bot.call(msg).then(move |result| {
                assert!(result.is_ok());
                Ok(())
            }))
        }));
    }

    #[test]
    fn test_send_video() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token.clone()).unwrap();
        tokio::run(futures::lazy(move || {
            let msg = SendVideo::builder()
                .chat_id(chat_id)
                .file("test_media/video/crab.mp4".to_string())
                .caption("Test SendVideo".to_string())
                .build();

            spawn(bot.call(msg).then(move |result| {
                assert!(result.is_ok());
                Ok(())
            }))
        }));
    }

    #[test]
    fn test_send_animation() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token.clone()).unwrap();
        tokio::run(futures::lazy(move || {
            let msg = SendAnimation::builder()
                .chat_id(chat_id)
                .file("test_media/video/crab.gif".to_string())
                .caption("Test SendAnimation".to_string())
                .build();

            spawn(bot.call(msg).then(move |result| {
                assert!(result.is_ok());
                Ok(())
            }))
        }));
    }

    #[test]
    fn test_send_voice() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token.clone()).unwrap();
        tokio::run(futures::lazy(move || {
            let msg = SendVoice::builder()
                .chat_id(chat_id)
                .file("test_media/audio/voice_message.ogg".to_string())
                .caption("Test SendVoice".to_string())
                .build();

            spawn(bot.call(msg).then(move |result| {
                assert!(result.is_ok());
                Ok(())
            }))
        }));
    }

    #[test]
    fn test_send_video_note() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token.clone()).unwrap();
        tokio::run(futures::lazy(move || {
            let msg = SendVideoNote::builder()
                .chat_id(chat_id)
                .file("test_media/video/crab.mp4".to_string())
                .build();

            spawn(bot.call(msg).then(move |result| {
                assert!(result.is_ok());
                Ok(())
            }))
        }));
    }

    #[test]
    fn test_send_location() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token).unwrap();

        tokio::run(futures::lazy(move || {
            let msg = SendLocation::builder()
                .chat_id(chat_id)
                .latitude(51.980448)
                .longitude(4.410606)
                .build();

            spawn(bot.call(msg).then(|result| {
                assert!(result.is_ok());
                Ok(())
            }))
        }));
    }

    #[test]
    fn test_send_live_location() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token).unwrap();

        tokio::run(futures::lazy(move || {
            let msg = SendLocation::builder()
                .chat_id(chat_id)
                .latitude(51.980448)
                .longitude(4.410606)
                .live_period(60)
                .build();

            spawn(bot.call(msg).then(|result| {
                assert!(result.is_ok());
                Ok(())
            }))
        }));
    }

    #[test]
    fn test_edit_live_location() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token).unwrap();
        let bot2 = bot.clone();

        tokio::run(futures::lazy(move || {
            let msg = SendLocation::builder()
                .chat_id(chat_id)
                .latitude(51.980448)
                .longitude(4.410606)
                .live_period(60)
                .build();

            spawn(bot.call(msg).then(move |result| {
                assert!(result.is_ok());

                let msg = EditMessageLiveLocation::builder()
                    .chat_id(chat_id)
                    .message_id(result.unwrap().message_id)
                    .latitude(50.980448)
                    .longitude(4.410606)
                    .build();
                spawn(bot2.call(msg).then(move |result| {
                    assert!(result.is_ok());
                    Ok(())
                }))
            }))
        }));
    }

    #[test]
    fn test_stop_live_location() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token).unwrap();
        let bot2 = bot.clone();

        tokio::run(futures::lazy(move || {
            let msg = SendLocation::builder()
                .chat_id(chat_id)
                .latitude(51.980448)
                .longitude(4.410606)
                .live_period(60)
                .build();

            spawn(bot.call(msg).then(move |result| {
                assert!(result.is_ok());

                let msg = StopMessageLiveLocation::builder()
                    .chat_id(chat_id)
                    .message_id(result.unwrap().message_id)
                    .build();
                spawn(bot2.call(msg).then(move |result| {
                    assert!(result.is_ok());
                    Ok(())
                }))
            }))
        }));
    }

    #[test]
    fn test_send_venue() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token).unwrap();

        tokio::run(futures::lazy(move || {
            let msg = SendVenue::builder()
                .chat_id(chat_id)
                .latitude(51.962881)
                .longitude(4.393794)
                .address("Rotterdamseweg 480")
                .title("Aan de Zweth")
                .build();

            spawn(bot.call(msg).then(|result| {
                assert!(result.is_ok());
                Ok(())
            }))
        }));
    }

    #[test]
    fn test_send_contact() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token).unwrap();

        tokio::run(futures::lazy(move || {
            let msg = SendContact::builder()
                .chat_id(chat_id)
                .phone_number("0612345678")
                .first_name("Deniz")
                .build();

            spawn(bot.call(msg).then(|result| {
                assert!(result.is_ok());
                Ok(())
            }))
        }));
    }

    #[test]
    fn test_send_poll() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token).unwrap();

        tokio::run(futures::lazy(move || {
            let msg = SendPoll::builder()
                .chat_id(chat_id)
                .question("Is this a useful test?".to_string())
                .options(vec![
                    "Yes".to_string(),
                    "Yes".to_string(),
                    "Maybe".to_string(),
                ])
                .build();

            spawn(bot.call(msg).then(|result| {
                assert!(result.is_ok());
                Ok(())
            }))
        }));
    }

    #[test]
    fn test_send_chat_action() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token).unwrap();

        tokio::run(futures::lazy(move || {
            let msg = SendChatAction::builder()
                .chat_id(chat_id)
                .action(ChatAction::Typing)
                .build();

            spawn(bot.call(msg).then(|result| {
                assert!(result.is_ok());
                Ok(())
            }))
        }));
    }

    #[test]
    fn test_get_user_profile_photos() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token).unwrap();

        tokio::run(futures::lazy(move || {
            let msg = GetUserProfilePhotos::builder().user_id(chat_id).build();

            spawn(bot.call(msg).then(|result| {
                assert!(result.is_ok());
                println!("Profile Photo Count: {:?}", result.unwrap().photos.len());
                Ok(())
            }))
        }));
    }

    #[test]
    fn test_get_file() {
        let (token, chat_id) = setup();
        let bot = Bot::new(&token.clone()).unwrap();
        let bot2 = bot.clone();

        tokio::run(futures::lazy(move || {
            let msg = SendPhoto::builder()
                .chat_id(chat_id)
                .file("test_media/photos/crab.png".to_string())
                .caption("Test SendPhoto".to_string())
                .build();

            spawn(bot.call(msg).then(move |result| {
                assert!(result.is_ok());
                let file_id = &result.unwrap().photo.unwrap()[0].file_id;
                let msg = GetFile::builder().file_id(file_id.to_owned()).build();

                spawn(bot2.call(msg).then(move |result| {
                    assert!(result.is_ok());
                    println!("{:?}", result.unwrap());
                    Ok(())
                }))
            }))
        }));
    }
}
