use std::collections::HashMap;
use std::sync::mpsc;

use command::Command;
use objects::update::Update;
use bot::Bot;
use updater::RUNNING;

pub struct CommandHandler {
    commands: HashMap<String, Box<Command>>,
}

impl CommandHandler {
    pub fn new() -> Self {
        CommandHandler { commands: HashMap::new() }
    }

    pub fn add<C: Command>(&mut self, command_name: &str, command: C) {
        self.commands.insert(command_name.to_string(), Box::new(command));
    }

    pub fn start_command_handling(&mut self, rx: mpsc::Receiver<Update>, bot: &Bot) {
        while RUNNING.with(|s| s.get()) {
            let update = rx.recv().unwrap();

            if let Some(t) = update.clone().message.and_then(|t| t.text) {
                if t.starts_with("/") {
                    let (_, bot_command) = t.split_whitespace().next().unwrap().split_at(1);
                    let name_command: Vec<&str> = bot_command.split("@").collect();

                    if name_command.len() == 1 ||
                       name_command.len() == 2 && name_command[1] == bot.username {
                        let command = self.commands.get_mut(name_command[0]);

                        if let Some(c) = command {
                            c.execute(bot, update);
                        }
                    }
                }
            }
        }
    }
}
