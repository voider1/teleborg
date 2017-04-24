# teleborg
A loose Telegram bot API for Rust based on the traiting system.
This project is inspired by [python-telegram-bot](https://github.com/python-telegram-bot/python-telegram-bot).

How to use the project
======================
Add this to your `Cargo.toml`
``` toml
[dependencies]
teleborg = "0.1.32"
```
It's on crates.io now, check it out https://crates.io/crates/teleborg.
Note that this project only works on Rust 1.16 or above.

An example
==========
Here we'll show you the bare minimum needed to register a command which sends a hardcoded reply when issued.

```Rust
extern crate teleborg;
use teleborg::{Dispatcher, Bot, Updater};
use teleborg::objects::Update;

fn main() {
    // Make sure you have your token
    let bot_token = "bot_token".to_string();
    // Creating a dispatcher which registers all the command and message handlers
    let mut dispatcher = Dispatcher::new();
    // Registering our command which we create below in the form as a function
    dispatcher.add_command_handler("test", test, false);
    // Start the updater, the Updater will start the threads, one of which will poll for updates
    // and send those to the Dispatcher's thread which will act upon it with the registered handlers
    Updater::start(Some(bot_token), None, None, None, dispatcher);
}

// Our first command handler
fn test(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
    bot.reply_to_message(&update, "It works!").unwrap();
}
```

Currently I only support send_message, reply_to_message and forward_message. More is to come.
I recommend not putting your token in the code, if you pass None as teleborg::updater::Updater::Start()'s first argument it'll automatically search for a environment variable called "TELEGRAM_BOT_TOKEN".
Just make sure you set the environment variable equal to your bot token and all is good.
