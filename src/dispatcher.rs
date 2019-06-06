use crate::{bot::Bot, command::Command, types::Update};
use std::{
    collections::HashMap,
    fmt::{self, Debug},
    sync::Arc,
};

#[derive(Debug)]
pub enum HasArgs {
    Yes,
    No,
}

/// A `Dispatcher` which will receive updates from the `Updater` and dispatches
/// them to the registered handlers.
///
/// You can add your command and message handlers to the `Dispatcher`.
#[derive(Debug, Default)]
pub struct Dispatcher {
    command_handlers: HashMap<String, (Box<dyn Command>, HasArgs)>,
    message_handlers: Vec<Box<dyn Command>>,
}

impl Dispatcher {
    /// Constructs a new `Dispatcher`.
    pub fn new() -> Self {
        Dispatcher {
            command_handlers: HashMap::new(),
            message_handlers: Vec::new(),
        }
    }

    /// Add a function which implements the `Command` trait to the `Dispatcher.command_handlers`.
    pub fn add_command_handler<C: Command>(
        &mut self,
        command_name: &str,
        command: C,
        has_args: HasArgs,
    ) {
        self.command_handlers
            .insert(command_name.to_string(), (Box::new(command), has_args));
    }

    /// Add a function which implements the `Command` trait to the `Dispatcher.message_handlers`.
    pub fn add_message_handler<C: Command>(&mut self, command: C) {
        self.message_handlers.push(Box::new(command));
    }

    /// Starts the update handling process and dispatches all the updates to the assigned handlers.
    pub fn handle(&mut self, bot: &Arc<Bot>, update: Update) {
        let username = &bot
            .username
            .as_ref()
            .expect("A bot should have a username, this can't be None");

        for message_handler in &mut self.message_handlers {
            message_handler.execute(&bot, update.clone(), None);
        }

        if let Some(t) = update.clone().message.and_then(|t| t.text) {
            if t.starts_with('/') {
                let msg = t.split_whitespace().collect::<Vec<&str>>();
                let (_, bot_command) = msg[0].split_at(1);
                let name_command = bot_command.split('@').collect::<Vec<&str>>();

                if name_command.len() == 1
                    || name_command.len() == 2 && &name_command[1] == username
                {
                    let command = self.command_handlers.get_mut(name_command[0]);

                    if let Some(c) = command {
                        match c.1 {
                            HasArgs::Yes => {
                                let args = msg.clone().split_off(1);
                                c.0.execute(bot, update, Some(args));
                            }
                            HasArgs::No => c.0.execute(bot, update, None),
                        }
                    }
                }
            }
        }
    }
}

impl Debug for dyn Command + 'static {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}
