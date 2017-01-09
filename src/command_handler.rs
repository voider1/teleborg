use std::collections::HashMap;
use std::sync::mpsc;
use std::collections::VecDeque;

use command::Command;
use updater::Updater;
use objects::update::Update;
use bot::Bot;

pub struct CommandHandler {
    commands: HashMap<String, Box<Command>>,
}

impl CommandHandler {
    pub fn new() -> Self {
        CommandHandler { commands: HashMap::new() }
    }

    pub fn add<C: Command>(&mut self, command_name: &str, command: C) {
        self.commands.insert(command_name.to_string(), Box::new(command));
        println!("Added command!");
    }

    pub fn start_command_handling(&mut self, rx: mpsc::Receiver<Update>, bot: &Bot) {
        println!("called startcommandhandling!");
        loop {
            let update = rx.recv().unwrap();

            if let Some(t) = update.clone().message.and_then(|t| t.text) {
                if t.starts_with("/") {
                    let (_, bot_command) = t.split_whitespace().next().unwrap().split_at(1);
                    let command = self.commands.get_mut(bot_command);
                    println!("{}", bot_command);

                    if let Some(c) = command {
                        c.execute(bot, update);
                    }
                }
            }
        }
    }
}
