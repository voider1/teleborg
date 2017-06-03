extern crate teleborg;

#[cfg(test)]
mod tests {
    use teleborg::{Dispatcher, Updater, Bot};
    use teleborg::objects::Update;
    use teleborg::objects::inline_query_results::{InlineQueryResultArticle, InlineQueryResult};
    use teleborg::objects::input_message_content::{InputTextMessageContent, InputLocationMessageContent};
    use teleborg::{ParseMode};

    #[test]
    fn test_updater() {
        let mut dispatcher = Dispatcher::new();
        dispatcher.add_command_handler("test", test, false);
        dispatcher.add_inline_query_handler(test_inline_query);
        Updater::start(None, None, None, None, dispatcher);
    }

    fn test(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        bot.reply_to_message(&update, "It works!").unwrap();
    }

    fn test_inline_query(bot: &Bot, update: Update, _: Option<Vec<&str>>) {
        if let Some(inline_query) = update.clone().inline_query {
            let mut results: Vec<Box<InlineQueryResult>> = Vec::new();
            let content = InputTextMessageContent::new("Telegram rules".to_owned(),
                                                       Some(ParseMode::Text),
                                                       Some(true));
            let article = InlineQueryResultArticle::new("Telegram text?".to_string(),
                                                        Box::new(content),
                                                        None,
                                                        None,
                                                        None,
                                                        None,
                                                        Some("https://telegram.org/img/t_logo.png".to_string()),
                                                        None,
                                                        None);

            let content2 = InputLocationMessageContent::new(35.0, 38.0);
            let article2 = InlineQueryResultArticle::new("Telegram location?".to_string(),
                                                         Box::new(content2),
                                                         None,
                                                         None,
                                                         None,
                                                         None,
                                                         Some("https://telegram.org/img/t_logo.png".to_string()),
                                                         None,
                                                         None);

            let article = Box::new(article);
            let article2 = Box::new(article2);
            results.push(article);
            results.push(article2);

            let answer_result = bot.answer_inline_query(&update, results);
            println!("{:?}", answer_result.err());
        }
    }
}
