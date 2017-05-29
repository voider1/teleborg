extern crate teleborg;

#[cfg(test)]
mod tests {
    use teleborg::{Dispatcher, Updater, Bot};
    use teleborg::objects::Update;
    use teleborg::objects::inline_query_results::InlineQueryResultArticle;
    use teleborg::objects::input_message_content::InputTextMessageContent;
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

    fn test_inline_query(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        if let Some(inline_query) = update.clone().inline_query {
            let mut results = Vec::new();
            let content = InputTextMessageContent::new("Test Message".to_owned(), Some(ParseMode::Text), Some(true));
            let article = InlineQueryResultArticle::new("Results".to_string(), content);
            results.push(article);
            let answer_result = bot.answer_inline_query(&update, results);
            assert!(answer_result.is_ok());
        }
    }
}
