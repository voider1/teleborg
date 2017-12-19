use std::{env, thread, time};
use std::sync::mpsc;
use std::sync::Arc;

use bot;
use dispatcher::Dispatcher;
use objects::Update;

const BASE_URL: &'static str = "https://api.telegram.org/bot";

/// An `Updater` which will request updates from the API.
///
/// The `Updater` is the entry point of this library and will start threads
/// which will poll for updates and dispatch them to the handlers.
pub struct Updater {
    token: String,
    poll_interval: u64,
    timeout: i32,
    network_delay: f32,
}

pub struct UpdaterBuilder {
    token: String,
    dispatcher: Dispatcher,
    poll_interval: u64,
    timeout: i32,
    network_delay: f32,
}

impl Updater {
    /// Constructs a new `UpdaterBuilder`.
    pub fn new(token: String, dispatcher: Dispatcher) -> UpdaterBuilder {
        let poll_interval = 0;
        let timeout = 10;
        let network_delay = 0.0;

        UpdaterBuilder {
            token,
            dispatcher,
            poll_interval,
            timeout,
            network_delay,
        }
    }

    /// Constructs a new `Updater` and starts the threads, if token is `None` it will check the
    /// environtment for the `TELEGRAM_BOT_TOKEN`.
    pub fn start(self, mut dispatcher: Dispatcher) {
        debug!("Starting updater...");
        let (tx, rx) = mpsc::channel();
        let bot = Arc::new(bot::Bot::new([BASE_URL, &self.token].concat()).unwrap());
        let updater_bot = bot.clone();
        let dispatcher_bot = bot.clone();

        thread::Builder::new()
            .name("dispatcher".to_string())
            .spawn(move || {
                dispatcher.start_handling(rx, dispatcher_bot);
            })
            .unwrap();

        thread::Builder::new()
            .name("updater".to_string())
            .spawn(move || {
                self.start_polling_thread(updater_bot, tx);
            })
            .unwrap()
            .join()
            .unwrap();
    }

    /// The method which will run in a thread and push the updates to the `Dispatcher`.
    fn start_polling_thread(&self, bot: Arc<bot::Bot>, tx: mpsc::Sender<Update>) {
        debug!("Going to start polling thread...");
        let poll_interval = time::Duration::from_secs(self.poll_interval);
        let mut last_update_id = 0;

        loop {
            let updates = bot.get_updates(last_update_id, None, self.timeout, self.network_delay);

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
                Err(e) => {
                    // Handle error
                    debug!("Error while polling updates: {:?}", e);
                    continue;
                }
            };

            thread::sleep(poll_interval);
        }
    }
}

impl UpdaterBuilder {
    pub fn poll_interval(&mut self, amount: u64) -> &mut Self {
        self.poll_interval = amount;
        self
    }

    pub fn timeout(&mut self, amount: i32) -> &mut Self {
        self.timeout = amount;
        self
    }

    pub fn network_delay(&mut self, amount: f32) -> &mut Self {
        self.network_delay = amount;
        self
    }

    pub fn build(self) -> (Updater, Dispatcher) {
        let UpdaterBuilder {
            token,
            dispatcher,
            poll_interval,
            timeout,
            network_delay,
        } = self;

        (
            Updater {
                token,
                poll_interval,
                timeout,
                network_delay,
            },
            dispatcher,
        )
    }

    pub fn start(self) {
        let (updater, dispatcher) = self.build();
        updater.start(dispatcher);
    }
}
