use std::collections::HashMap;
use std::sync::{Arc, mpsc};

use command::Command;
use objects::update::Update;
use bot::Bot;

pub struct Dispatcher {
    command_handlers: HashMap<String, (Box<Command>, bool)>,
    message_handlers: Vec<Box<Command>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher {
            command_handlers: HashMap::new(),
            message_handlers: Vec::new(),
        }
    }

    pub fn add_command_handler<C: Command>(&mut self, command_name: &str, command: C, args: bool) {
        self.command_handlers.insert(command_name.to_string(), (Box::new(command), args,));
    }

    pub fn add_message_handler<C: Command>(&mut self, command: C) {
        self.message_handlers.push(Box::new(command));
    }

    pub fn start_handling(&mut self, rx: mpsc::Receiver<Update>, bot: Arc<Bot>) {
        loop {
            let update = rx.recv().unwrap();

            if let Some(t) = update.clone().message.and_then(|t| t.text) {
                if t.starts_with("/") {
                    let msg = t.split_whitespace().collect::<Vec<&str>>();
                    let (_, bot_command) = msg[0].split_at(1);
                    let name_command = bot_command.split("@").collect::<Vec<&str>>();

                    if name_command.len() == 1 ||
                       name_command.len() == 2 && name_command[1] == bot.username {
                        let command = self.command_handlers.get_mut(name_command[0]);

                        if let Some(c) = command {
                            if c.1 {
                                let args = msg.clone().split_off(1);
                                c.0.execute(&bot, update, Some(args));
                            } else {
                                c.0.execute(&bot, update, None);
                            }
                            continue;
                        }
                    }
                }
            }
            for message_handler in self.message_handlers.iter_mut() {
                message_handler.execute(&bot, update.clone(), None);
            }
        }
    }
}
