# Teleborg
A loose Telegram bot API in Rust.

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

use std::env;

use teleborg::{Bot, Dispatcher, Updater};
use teleborg::types::Update;
use teleborg::methods::{Method, SendMessage};

fn main() {
    // Get bot your token from the environment
    let token = env::var("TELEGARM_BOT_TOKEN").expect("No token found");
    // Creating a dispatcher which registers all the command and message handlers
    let mut dispatcher = Dispatcher::new();
    // Registering our command which we create below in the form as a function
    dispatcher.add_command_handler("test", test, false);
    // Create an Updater builder and configure it as you like, after that build it and start it.
    // The Updater will start the threads, one of which will poll for updates
    // and send those to the Dispatcher's thread which will act upon it with the registered handlers
    Updater::builder().token(token).build().start(dispatcher);
}

// Our first command handler
fn test(bot: &Bot, update: Update, _: Option<Vec<&str>>) {
    let chat_id = update.message.unwrap().chat.id;
    let text = "It works!";

    SendMessage::builder()
        .chat_id(chat_id)
        .text(text)
        .build()
        .call(&bot)
        .unwrap();
}
```
