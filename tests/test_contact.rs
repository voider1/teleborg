extern crate teleborg;

#[cfg(test)]
mod tests {
    use teleborg::objects::update::Update;
    use teleborg::updater;

    // This test tries to send a contact by sending a
    // simple phone number (+1 123-456-7890) along with
    // "Test Contact" as the first/last name for the contact.
    fn test_contact(bot:&Bot, update: Update) {
        let chat_id = &update.message.unwrap().chat.id;
        let contact = bot.send_contact(chat_id,
                                 "+1 123-456-7890",
                                 "Test",
                                 "Contact",
                                 None,
                                 None);
        assert!(contact.is_ok());
    }
}
