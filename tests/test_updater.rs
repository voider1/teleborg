extern crate teleborg;

#[cfg(test)]
mod tests {
    use teleborg::{Dispatcher, Updater, Bot};
    use teleborg::objects::Update;
    use teleborg::objects::inline_query_results::{InlineQueryResult, InlineQueryResultArticle,
                                                  InlineQueryResultPhoto, InlineQueryResultGif,
                                                  InlineQueryResultMpeg4Gif,
                                                  InlineQueryResultVideo, InlineQueryResultAudio,
                                                  InlineQueryResultVoice,
                                                  InlineQueryResultDocument,
                                                  InlineQueryResultLocation,
                                                  InlineQueryResultVenue,
                                                  InlineQueryResultContact, InlineQueryResultGame};
    use teleborg::objects::input_message_content::{InputTextMessageContent,
                                                   InputLocationMessageContent,
                                                   InputContactMessageContent,
                                                   InputVenueMessageContent};
    use teleborg::ParseMode;

    #[test]
    fn test_updater() {
        let mut dispatcher = Dispatcher::new();
        dispatcher.add_command_handler("test", test, false);
        dispatcher.add_inline_query_handler(test_inline_query);
        dispatcher.add_callback_query_handler(test_callback_queries);
        Updater::start(None, None, None, None, dispatcher);
    }

    fn test(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        bot.reply_to_message(&update, "It works!").unwrap();
    }

    fn test_inline_query(bot: &Bot, update: Update, _: Option<Vec<&str>>) {
        if let Some(inline_query) = update.clone().inline_query {
            let mut results: Vec<Box<InlineQueryResult>> = Vec::new();
            let content_text = InputTextMessageContent::new("Telegram rules".to_owned(),
                                                            Some(ParseMode::Text),
                                                            Some(true));
            let article_text =
                InlineQueryResultArticle::new("Telegram text?".to_string(),
                                              Box::new(content_text),
                                              None,
                                              None,
                                              None,
                                              None,
                                              Some("https://telegram.org/img/t_logo.png"
                                                       .to_string()),
                                              None,
                                              None);

            let content_location = InputLocationMessageContent::new(35.0, 38.0);
            let article_location =
                InlineQueryResultArticle::new("Telegram location?".to_string(),
                                              Box::new(content_location),
                                              None,
                                              None,
                                              None,
                                              None,
                                              Some("https://telegram.org/img/t_logo.png"
                                                       .to_string()),
                                              None,
                                              None);
            let content_contact = InputContactMessageContent::new(String::from("06-12345678"),
                                                                  String::from("Voider1"),
                                                                  None);
            let article_contact =
                InlineQueryResultArticle::new("Telegram contact?".to_string(),
                                              Box::new(content_contact),
                                              None,
                                              None,
                                              None,
                                              None,
                                              Some("https://telegram.org/img/t_logo.png"
                                                       .to_string()),
                                              None,
                                              None);
            let content_venue = InputVenueMessageContent::new(35.0,
                                                              38.0,
                                                              "test title".to_string(),
                                                              "test address".to_string(),
                                                              None);
            let article_venue =
                InlineQueryResultArticle::new("Telegram venue?".to_string(),
                                              Box::new(content_venue),
                                              None,
                                              None,
                                              None,
                                              None,
                                              Some("https://telegram.org/img/t_logo.png"
                                                       .to_string()),
                                              None,
                                              None);
            let photo =
                InlineQueryResultPhoto::new("https://telegram.org/img/t_logo.png".to_string(),
                                            "https://telegram.org/img/t_logo.png".to_string(),
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None);

            let gif =
                InlineQueryResultGif::new("https://media.tenor.com/images/53d5b0e9fadc7a6d8325a2aa02cfbffa/tenor.gif".to_string(),
                                          None,
                                          None,
                                          None,
                                          "https://media.tenor.com/images/53d5b0e9fadc7a6d8325a2aa02cfbffa/tenor.gif".to_string(),
                                          None,
                                          None,
                                          None,
                                          None);

            let mpeg4 =
                InlineQueryResultMpeg4Gif::new("https://media3.giphy.com/media/N04Fkkzhf9slO/giphy.mp4".to_string(),
                                               None,
                                               None,
                                               None,
                                               "https://media3.giphy.com/media/N04Fkkzhf9slO/giphy.mp4".to_string(),
                                               None,
                                               None,
                                               None,
                                               None);

            let video =
                InlineQueryResultVideo::new("https://media1.giphy.com/media/1IuRitRjkbgTC/giphy.mp4"
                                                .to_string(),
                                            "video/mp4".to_string(),
                                            "https://i.ytimg.com/vi/Ani_6IRV20A/hqdefault.jpg"
                                                .to_string(),
                                            "Fried noodles".to_string(),
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None);

            let audio = InlineQueryResultAudio::new("http://www.kozco.com/tech/piano2-CoolEdit.mp3"
                                                        .to_string(),
                                                    "Epic Piano".to_string(),
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None);

            let voice = InlineQueryResultVoice::new("https://ia902304.us.archive.org/35/items/ogg_test/testogg.ogg".to_string(),
                                                    "Test Ogg".to_string(),
                                                    None,
                                                    None,
                                                    None,
                                                    None);

            let document = InlineQueryResultDocument::new("Test Doc".to_string(),
                                                          None,
                                                          "http://www.colorado.edu/conflict/peace/download/peace_essay.ZIP".to_string(),
                                                          "application/zip".to_string(),
                                                          None,
                                                          None,
                                                          None,
                                                          None,
                                                          None,
                                                          None);

            let location = InlineQueryResultLocation::new(35.0,
                                                          38.0,
                                                          "Location Test".to_string(),
                                                          None,
                                                          None,
                                                          None,
                                                          None,
                                                          None);

            let venue = InlineQueryResultVenue::new(35.0,
                                                    38.0,
                                                    "Test Venue".to_string(),
                                                    "Test address".to_string(),
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None);

            let contact = InlineQueryResultContact::new("06-12345678".to_string(),
                                                        "Voider1".to_string(),
                                                        None,
                                                        None,
                                                        None,
                                                        None,
                                                        None,
                                                        None);

            let game = InlineQueryResultGame::new("floppy_bird".to_string(), None);

            results.push(Box::new(article_text));
            results.push(Box::new(article_location));
            results.push(Box::new(article_contact));
            results.push(Box::new(article_venue));
            results.push(Box::new(photo));
            results.push(Box::new(gif));
            results.push(Box::new(mpeg4));
            results.push(Box::new(video));
            results.push(Box::new(audio));
            results.push(Box::new(voice));
            results.push(Box::new(document));
            results.push(Box::new(location));
            results.push(Box::new(venue));
            results.push(Box::new(contact));
            results.push(Box::new(game));

            let answer_result = bot.answer_inline_query(&update,
                                                        results,
                                                        None,
                                                        None,
                                                        None,
                                                        None,
                                                        None);
            assert!(answer_result.is_ok());
        }
    }

    fn test_callback_queries(bot: &Bot, update: Update, _: Option<Vec<&str>>) {
        println!("{:?}", update.callback_query);
        let result = bot.answer_callback_query(&update,
                                               Some("It works!!!".to_string()),
                                               Some(true),
                                               Some("https://github.com/voider1/teleborg"
                                                        .to_string()),
                                               Some(10));
        assert!(result.is_ok());
    }
}
