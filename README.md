# Teleborg
A loose Telegram bot API in Rust. Fully async with Tokio.

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

```rust
use std::env;

use teleborg::{methods::SendMessage, spawn, types::Update, Bot, Dispatcher, Future, Updater};

fn main() {
    // Get bot your token from the environment
    let token = env::var("TELEGARM_BOT_TOKEN").expect("No token found");
    // Creating a dispatcher which registers all the command and message handlers
    let mut dispatcher = Dispatcher::new();
    // Registering our command which we create below in the form as a function
    dispatcher.add_command_handler("test", test, false);
    // Create an Updater builder and configure it as you like, after that build it and start it.
    // The Updater will start the Tokio runtime, this ensures you can spawn tasks inside of
    // your command handlers.
    Updater::builder().build(token).start(dispatcher);
}

// Our first command handler
fn test(bot: &Bot, update: Update, _: Option<Vec<&str>>) {
    let chat_id = update.message.unwrap().chat.id;
    let text = "It works!";

    let msg = SendMessage::builder().chat_id(chat_id).text(text).build();

    spawn(bot.call(&msg).then(|_| Ok(())));
}
```
