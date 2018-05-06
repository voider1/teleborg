use std::{thread, time};
use std::sync::mpsc;
use std::sync::Arc;

use bot;
use dispatcher::Dispatcher;
use types::Update;

const BASE_URL: &str = "https://api.telegram.org/bot";

/// An `Updater` which will keep track of the updates from the API.
///
/// The `Updater` is the entry point of this library and will start the threads
/// which will poll for updates and dispatch them to the handlers.
#[derive(Debug, TypedBuilder)]
pub struct Updater {
    /// The token of your bot for authentication.
    token: String,
    /// Amount of seconds to wait between polling the Telegram server.
    #[default = "0"]
    poll_interval: u64,
    /// Amount of time to wait for a request until trying again.
    #[default = "10"]
    timeout: i32,
}

impl Updater {
    /// Constructs a new `Updater` and starts the threads.
    pub fn start(self, mut dispatcher: Dispatcher) {
        debug!("Starting updater...");
        let (tx, rx) = mpsc::channel();
        let bot = Arc::new(bot::Bot::new([BASE_URL, &self.token].concat()).unwrap());
        let updater_bot = Arc::clone(&bot);
        let dispatcher_bot = Arc::clone(&bot);

        thread::Builder::new()
            .name("dispatcher".to_string())
            .spawn(move || {
                dispatcher.start_handling(&rx, &dispatcher_bot);
            })
            .unwrap();

        thread::Builder::new()
            .name("updater".to_string())
            .spawn(move || {
                self.start_polling_thread(&updater_bot, &tx);
            })
            .unwrap()
            .join()
            .unwrap();
    }

    /// The method which will run in a thread and push the updates to the `Dispatcher`.
    fn start_polling_thread(&self, bot: &Arc<bot::Bot>, tx: &mpsc::Sender<Update>) {
        debug!("Going to start polling thread...");
        let poll_interval = time::Duration::from_secs(self.poll_interval);
        let mut last_update_id = 0;

        loop {
            let updates = bot.get_updates(last_update_id, None, self.timeout);

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
