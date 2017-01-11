use std::{env, thread, time};
use std::sync::mpsc;

use reqwest;
use crossbeam;

use bot;
use command_handler::CommandHandler;
use update::Update;

const BASE_URL: &'static str = "https://api.telegram.org/bot";

pub struct Updater {
    token: String,
    last_update_id: i32,
    pub running: bool,
    pub is_idle: bool,
}

impl Updater {
    /// Creates a new Updater struct.
    pub fn start(token: Option<String>,
                 poll_interval: Option<u64>,
                 timeout: Option<i32>,
                 network_delay: Option<i32>,
                 command_handler: CommandHandler) {
        let token = token.or_else(|| env::var("TELEGRAM_BOT_TOKEN").ok())
            .expect("You should pass in a token to new or set TELEGRAM_BOT_TOKEN");

        let updater = Updater {
            token: token,
            last_update_id: 0,
            running: false,
            is_idle: false,
        };

        updater.start_polling(poll_interval, timeout, network_delay, command_handler);
    }

    pub fn start_polling(mut self,
                         poll_interval: Option<u64>,
                         timeout: Option<i32>,
                         network_delay: Option<i32>,
                         mut command_handler: CommandHandler) {
        if !self.running {
            self.running = true;

            let (tx, rx) = mpsc::channel();
            let bot = bot::Bot::new([BASE_URL, &self.token].concat()).unwrap();

            // Spawn scoped threads
            crossbeam::scope(|scope| {
                scope.spawn(|| {
                    self.start_polling_thread(poll_interval, timeout, network_delay, &bot, tx)
                });
                scope.spawn(|| {
                    command_handler.start_command_handling(rx, &bot);
                });
            });
        }
    }

    fn start_polling_thread(&mut self,
                            poll_interval: Option<u64>,
                            timeout: Option<i32>,
                            network_delay: Option<i32>,
                            bot: &bot::Bot,
                            tx: mpsc::Sender<Update>) {
        let poll_interval = time::Duration::from_secs(poll_interval.unwrap_or(0));

        while self.running {
            let updates = bot.get_updates(self.last_update_id, None, timeout, network_delay);

            match updates {
                Ok(Some(ref v)) => {
                    if let Some(u) = v.last() {
                        for update in v {
                            tx.send(update.clone()).unwrap();
                        }
                        let update_id_store = u.update_id + 1;
                        self.last_update_id = update_id_store as i32;
                    } else {
                        // Do nothing, the vector is empty
                        continue;
                    }
                }
                Ok(None) => {
                    // Do nothing, we have nothing
                    continue;
                }
                Err(err) => {
                    // Handle error
                    continue;
                }
            };

            thread::sleep(poll_interval);
        }
    }
}
