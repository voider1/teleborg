use std::{env, thread, time};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};
use std::thread::JoinHandle;

use bot;
use dispatcher::Dispatcher;
use update::Update;

const BASE_URL: &'static str = "https://api.telegram.org/bot";

pub struct Updater {
    token: String,
    updater_thread: Option<JoinHandle<()>>,
    dispatch_thread: Option<JoinHandle<()>>,
    pub is_running: Arc<AtomicBool>,
    pub is_idle: Arc<AtomicBool>,
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

        let mut updater = Updater {
            token: token,
            updater_thread: None,
            dispatch_thread: None,
            is_running: Arc::new(ATOMIC_BOOL_INIT),
            is_idle: Arc::new(ATOMIC_BOOL_INIT),
        };

        updater.start_polling(poll_interval, timeout, network_delay, dispatcher);
        updater
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    fn start_polling(&mut self,
                     poll_interval: Option<u64>,
                     timeout: Option<i32>,
                     network_delay: Option<i32>,
                     mut dispatcher: Dispatcher) {
        if !self.is_running.load(Ordering::Relaxed) {
            self.is_running.store(true, Ordering::SeqCst);

            let (tx, rx) = mpsc::channel();
            let bot = Arc::new(bot::Bot::new([BASE_URL, &self.token].concat()).unwrap());
            let updater_running = self.is_running.clone();
            let dispatcher_running = self.is_running.clone();
            let updater_bot = bot.clone();
            let dispatcher_bot = bot.clone();

            // Spawn threads
            self.updater_thread = Some(thread::Builder::new()
                .name("updater".to_string())
                .spawn(move || {
                    Self::start_polling_thread(updater_running,
                                               poll_interval,
                                               timeout,
                                               network_delay,
                                               updater_bot,
                                               tx);
                })
                .unwrap());
            self.dispatch_thread = Some(thread::Builder::new()
                .name("dispatcher".to_string())
                .spawn(move || {
                    dispatcher.start_command_handling(dispatcher_running, rx, dispatcher_bot);
                })
                .unwrap());
        }
    }

    fn start_polling_thread(is_running: Arc<AtomicBool>,
                            poll_interval: Option<u64>,
                            timeout: Option<i32>,
                            network_delay: Option<i32>,
                            bot: Arc<bot::Bot>,
                            tx: mpsc::Sender<Update>) {
        let poll_interval = time::Duration::from_secs(poll_interval.unwrap_or(0));
        let mut last_update_id = 0;

        while is_running.load(Ordering::SeqCst) {
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
