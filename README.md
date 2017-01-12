# teleborg
A loose Telegram bot API for Rust based on the traiting system. 
This project is inspired by [python-telegram-bot](https://github.com/python-telegram-bot/python-telegram-bot).

How to use the project
======================
Add this to your `Cargo.toml`
``` toml
[dependencies]
teleborg = { git = "https://github.com/voider1/teleborg" }
```
I will add this project to crates.io in the future when I've developed it some more.
Note that this project only works on Rust 1.16 or above, which currently is nightly.

A simple example
================
Let's show you how Teleborg works at its simplest, it's not yet very full fledged, but it can respond to commands.

```Rust
extern crate teleborg;
use teleborg::command_handler;
use teleborg::bot::Bot;
use teleborg::objects::update::Update;
use teleborg::updater;

fn main() {
    let mut commands = command_handler::CommandHandler::new();
    let bot_token = "bot_token".to_string();
    commands.add("test", test);
    updater::Updater::start(Some(bot_token), None, None, None, commands);
}

fn test(bot: &Bot, update: Update) {
    bot.reply_to_message(&update, "It works!").unwrap();
}
```

Currently I only support send_message, reply_to_message and forward_message. More is to come.
If you don't want to put your token in your code, just pass None for your bot token and export your token as environment variable with the name TELEGRAM_BOT_TOKEN. The library takes the environment variable out on its own.
