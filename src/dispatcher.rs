use std::collections::HashMap;
use std::sync::{Arc, mpsc};
use std::sync::atomic::{AtomicBool, Ordering};

use command::Command;
use objects::update::Update;
use bot::Bot;

pub struct Dispatcher {
    command_handlers: HashMap<String, Box<Command>>,
    message_handlers: Vec<Box<Command>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher {
            command_handlers: HashMap::new(),
            message_handlers: Vec::new(),
        }
    }

    pub fn add_command_handler<C: Command>(&mut self, command_name: &str, command: C) {
        self.command_handlers.insert(command_name.to_string(), Box::new(command));
    }

    pub fn add_message_handler<C: Command>(&mut self, command: C) {
        self.message_handlers.push(Box::new(command));
    }

    pub fn start_command_handling(&mut self,
                                  is_running: Arc<AtomicBool>,
                                  rx: mpsc::Receiver<Update>,
                                  bot: Arc<Bot>) {
        while is_running.load(Ordering::Relaxed) {
            let update = rx.recv().unwrap();

            if let Some(t) = update.clone().message.and_then(|t| t.text) {
                if t.starts_with("/") {
                    let (_, bot_command) = t.split_whitespace().next().unwrap().split_at(1);
                    let name_command: Vec<&str> = bot_command.split("@").collect();

                    if name_command.len() == 1 ||
                       name_command.len() == 2 && name_command[1] == bot.username {
                        let command = self.command_handlers.get_mut(name_command[0]);

                        if let Some(c) = command {
                            c.execute(&bot, update);
                            continue;
                        }
                    }
                }
            }
            for message_handler in self.message_handlers.iter_mut() {
                message_handler.execute(&bot, update.clone());
            }
        }
    }
}
