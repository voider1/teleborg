use std::collections::HashMap;
use std::sync::{mpsc, Arc};

use command::Command;
use bot::Bot;
use objects::Update;

/// A `Dispatcher` which will receive updates from the `Updater` and dispatches
/// them to the registered handlers.
///
/// You can add your command and message handlers to the `Dispatcher`.
#[derive(Default)]
pub struct Dispatcher {
    command_handlers: HashMap<String, (Box<Command>, bool)>,
    message_handlers: Vec<Box<Command>>,
}

impl Dispatcher {
    /// Constructs a new `Dispatcher`.
    pub fn new() -> Self {
        debug!("Going to construct a new Dispatcher...");
        Dispatcher {
            command_handlers: HashMap::new(),
            message_handlers: Vec::new(),
        }
    }

    /// Add a function which implements the `Command` trait to the `Dispatcher.command_handlers`.
    pub fn add_command_handler<C: Command>(&mut self, command_name: &str, command: C, args: bool) {
        self.command_handlers
            .insert(command_name.to_string(), (Box::new(command), args));
    }

    /// Add a function which implements the `Command` trait to the `Dispatcher.message_handlers`.
    pub fn add_message_handler<C: Command>(&mut self, command: C) {
        self.message_handlers.push(Box::new(command));
    }

    /// Starts the update handling process and dispatches all the updates to the assigned handlers.
    pub fn start_handling(&mut self, rx: &mpsc::Receiver<Update>, bot: &Arc<Bot>) {
        debug!("Going to start dispatcher thread...");
        loop {
            let update = rx.recv().unwrap();

            if let Some(t) = update.clone().message.and_then(|t| t.text) {
                if t.starts_with('/') {
                    let msg = t.split_whitespace().collect::<Vec<&str>>();
                    let (_, bot_command) = msg[0].split_at(1);
                    let name_command = bot_command.split('@').collect::<Vec<&str>>();

                    if name_command.len() == 1
                        || name_command.len() == 2 && name_command[1] == bot.username
                    {
                        let command = self.command_handlers.get_mut(name_command[0]);

                        if let Some(c) = command {
                            debug!("Going to execute the {} command...", name_command[0]);
                            if c.1 {
                                let args = msg.clone().split_off(1);
                                debug!("With these arguments: {:?}", args);
                                c.0.execute(bot, update, Some(args));
                            } else {
                                c.0.execute(bot, update, None);
                            }
                            continue;
                        }
                    }
                }
            }
            for message_handler in &mut self.message_handlers {
                debug!("Going to execute a message handler...");
                message_handler.execute(bot, update.clone(), None);
            }
        }
    }
}
