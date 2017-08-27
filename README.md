# Teleborg
A loose Telegram bot API for Rust based on the traiting system.

How to use the project
======================
Add this to your `Cargo.toml`
``` toml
[dependencies]
teleborg = "0.1.32"
```

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
fn test(bot: &Bot, update: Update, _: Option<Vec<&str>>) {
    bot.reply_to_message(&update, "It works!").unwrap();
}
```
