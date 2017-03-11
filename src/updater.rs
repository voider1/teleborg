use std::{env, thread, time};
use std::sync::mpsc;
use std::sync::Arc;

use bot;
use dispatcher::Dispatcher;
use update::Update;

const BASE_URL: &'static str = "https://api.telegram.org/bot";

pub struct Updater {
    token: String,
}

impl Updater {
    /// Creates a new Updater struct.
    pub fn start(token: Option<String>,
                 poll_interval: Option<u64>,
                 timeout: Option<i32>,
                 network_delay: Option<i32>,
                 dispatcher: Dispatcher)
                 -> Updater {
        let token = token.or_else(|| env::var("TELEGRAM_BOT_TOKEN").ok())
            .expect("You should pass in a token to new or set the TELEGRAM_BOT_TOKEN env var");

        let mut updater = Updater { token: token };

        updater.start_polling(poll_interval, timeout, network_delay, dispatcher);
        updater
    }

    fn start_polling(&mut self,
                     poll_interval: Option<u64>,
                     timeout: Option<i32>,
                     network_delay: Option<i32>,
                     mut dispatcher: Dispatcher) {
        let (tx, rx) = mpsc::channel();
        let bot = Arc::new(bot::Bot::new([BASE_URL, &self.token].concat()).unwrap());
        let updater_bot = bot.clone();
        let dispatcher_bot = bot.clone();

        // Spawn threads
        thread::Builder::new()
            .name("dispatcher".to_string())
            .spawn(move || {
                dispatcher.start_handling(rx, dispatcher_bot);
            })
            .unwrap();

        thread::Builder::new()
            .name("updater".to_string())
            .spawn(move || {
                Self::start_polling_thread(poll_interval, timeout, network_delay, updater_bot, tx);
            })
            .unwrap()
            .join()
            .unwrap();
    }

    fn start_polling_thread(poll_interval: Option<u64>,
                            timeout: Option<i32>,
                            network_delay: Option<i32>,
                            bot: Arc<bot::Bot>,
                            tx: mpsc::Sender<Update>) {
        let poll_interval = time::Duration::from_secs(poll_interval.unwrap_or(0));
        let mut last_update_id = 0;

        loop {
            let updates = bot.get_updates(last_update_id, None, timeout, network_delay);

            match updates {
                Ok(Some(ref v)) => {
                    if let Some(u) = v.last() {
                        for update in v {
                            tx.send(update.clone()).unwrap();
                        }
                        last_update_id = (u.update_id + 1) as i32;
                    } else {
                        // Do nothing, the vector is empty
                        continue;
                    }
                }
                Ok(None) => {
                    // Do nothing, we have nothing
                    continue;
                }
                Err(..) => {
                    // Handle error
                    continue;
                }
            };

            thread::sleep(poll_interval);
        }
    }
}
