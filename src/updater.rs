use std::{env, thread, time};
use std::sync::atomic::{AtomicBool, AtomicIsize, ATOMIC_BOOL_INIT, ATOMIC_ISIZE_INIT};
use std::sync::atomic::Ordering::Relaxed;

use reqwest;
use crossbeam;

use bot;
use update::Update;
use error::Result;

const BASE_URL: &'static str = "https://api.telegram.org/bot";

#[derive(Debug)]
pub struct Updater {
    token: String,
    bot: bot::Bot,
    last_update_id: AtomicIsize,
    running: AtomicBool,
    is_idle: AtomicBool,
    client: reqwest::Client,
}

impl Updater {
    /// Creates a new Updater struct.
    pub fn new(token: Option<String>) -> Result<Updater> {
        let token = token.or_else(|| env::var("TELEGRAM_BOT_TOKEN").ok())
            .expect("You should pass in a token to new or set TELEGRAM_BOT_TOKEN");

        let bot_url = [BASE_URL, &token].concat();
        let bot = bot::Bot::new(bot_url)?;
        let client = reqwest::Client::new()?;

        Ok(Updater {
            token: token,
            bot: bot,
            last_update_id: ATOMIC_ISIZE_INIT,
            running: ATOMIC_BOOL_INIT,
            is_idle: ATOMIC_BOOL_INIT,
            client: client,
        })
    }

    pub fn start_polling(&self,
                         poll_interval: Option<u64>,
                         timeout: Option<i32>,
                         network_delay: Option<i32>) {
        if !self.running.load(Relaxed) {
            self.running.store(true, Relaxed);

            crossbeam::scope(|scope| {
                scope.spawn(|| self.start_polling_thread(poll_interval, timeout, network_delay));
            });
        }
    }

    fn start_polling_thread(&self,
                            poll_interval: Option<u64>,
                            timeout: Option<i32>,
                            network_delay: Option<i32>) {
        let poll_interval = time::Duration::from_secs(poll_interval.unwrap_or(0));

        while self.running.load(Relaxed) {
            let updates = self.bot
                .get_updates(self.last_update_id.load(Relaxed) as i32,
                             None,
                             timeout,
                             network_delay);

            let last_update = match updates {
                Ok(Some(ref v)) => {
                    if let Some(u) = v.last() {
                        u
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

            self.last_update_id.store((last_update.update_id + 1) as isize, Relaxed);
            println!("{:?}", self.last_update_id.load(Relaxed));
            thread::sleep(poll_interval);
        }
    }
}
