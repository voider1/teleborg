use std::sync::Arc;

use futures::{Async, Poll, Stream};
use tokio;
use typed_builder::TypedBuilder;

use crate::bot::Bot;
use crate::dispatcher::Dispatcher;
use crate::types::Update;

/// An `Updater` which will keep track of the updates from the API.
///
/// The `Updater` is the entry point of this library and will start the threads
/// which will poll for updates and dispatch them to the handlers.
#[derive(Debug, TypedBuilder)]
pub struct Updater {
    /// Amount of seconds to wait between polling the Telegram server.
    #[builder(default = 0)]
    poll_interval: usize,
    /// Amount of time to wait for a request until trying again.
    #[builder(default = 10)]
    timeout: usize,
    #[builder(default = 0)]
    offset: usize,
    #[builder(default = 100)]
    limit: usize,
    /// Updates buffer
    updates: Vec<Update>,
    bot: Arc<Bot>,
}

impl Updater {
    /// Constructs a new `Updater` and starts the threads.
    pub fn start(mut self, token: &str, mut dispatcher: Dispatcher) {
        self.bot = Arc::new(Bot::new(token).unwrap());
        let bot = Arc::clone(&self.bot);
        let start = self.for_each(move |update| dispatcher.handle(&bot, update));
        tokio::run(start);
    }
}

impl Stream for Updater {
    type Item = Update;

    type Error = ();

    fn poll(&mut self) -> Poll<Option<Update>, ()> {
        let update = self.updates.pop();

        if let Some(u) = update {
            self.offset = (u.update_id + 1) as usize;
            Ok(Async::Ready(Some(u)))
        } else {
            let updates = self.bot.get_updates(self.offset, self.limit, self.timeout);

            if let Ok(Some(mut updates)) = updates {
                updates.reverse();
                self.updates.append(&mut updates);
                let u = self.updates.pop().unwrap();
                self.offset = (u.update_id + 1) as usize;
                Ok(Async::Ready(Some(u)))
            } else {
                Ok(Async::NotReady)
            }
        }
    }
}
