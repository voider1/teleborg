use std::sync::Arc;

use futures::{future, Async, Poll, Stream};
use tokio;

use crate::bot::Bot;
use crate::dispatcher::Dispatcher;
use crate::types::Update;

/// An `Updater` which will keep track of the updates from the API.
///
/// The `Updater` is the entry point of this library and will start the threads
/// which will poll for updates and dispatch them to the handlers.
#[derive(Debug)]
pub struct Updater {
    /// Amount of seconds to wait between polling the Telegram server.
    poll_interval: usize,
    /// Amount of time to wait for a request until trying again.
    timeout: usize,
    offset: usize,
    limit: usize,
    /// Updates buffer
    updates: Vec<Update>,
    bot: Arc<Bot>,
}

#[derive(Debug, Default)]
pub struct UpdaterBuilder {
    poll_interval: usize,
    timeout: usize,
    offset: usize,
    limit: usize,
}

impl Updater {
    /// Create a new UpdaterBuilder with sane defaults for configuration
    pub fn new() -> UpdaterBuilder {
        UpdaterBuilder {
            poll_interval: 0,
            timeout: 10,
            offset: 0,
            limit: 100,
        }
    }

    /// Constructs a new `Updater` and starts the threads.
    pub fn start(self, mut dispatcher: Dispatcher) {
        let bot = Arc::clone(&self.bot);
        let start = self.for_each(move |update| {
            dispatcher.handle(&bot, update);
            future::ok(())
        });

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

impl UpdaterBuilder {
    pub fn poll_interval(mut self, interval: usize) -> Self {
        self.poll_interval = interval;
        self
    }

    pub fn timeout(mut self, timeout: usize) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn build(self, token: &str) -> Updater {
        let Self {
            poll_interval,
            timeout,
            offset,
            limit,
        } = self;

        let bot = Arc::new(Bot::new(token).unwrap());
        let updates = Vec::new();

        Updater {
            poll_interval,
            timeout,
            offset,
            limit,
            updates,
            bot,
        }
    }
}
