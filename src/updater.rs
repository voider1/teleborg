use crate::{bot::Bot, dispatcher::Dispatcher, types::Update};
use failure::Error as FailureError;
use futures::{future, try_ready, Async, Future, Poll, Stream};
use std::{
    fmt::{self, Debug},
    sync::Arc,
};
use tokio;

/// An `Updater` which will keep track of the updates from the API.
///
/// The `Updater` is the entry point of this library and will start the threads
/// which will poll for updates and dispatch them to the handlers.
pub struct Updater {
    /// Amount of time to wait for a request until trying again.
    timeout: usize,
    offset: usize,
    limit: usize,
    update_future: Option<Box<dyn Future<Item = Vec<Update>, Error = FailureError> + Send>>,
    /// Updates buffer
    updates: Vec<Update>,
    bot: Arc<Bot>,
}

#[derive(Debug, Default)]
pub struct UpdaterBuilder {
    timeout: usize,
    offset: usize,
    limit: usize,
}

impl Updater {
    /// Create a new UpdaterBuilder with sane defaults for configuration
    pub fn builder() -> UpdaterBuilder {
        UpdaterBuilder {
            timeout: 10,
            offset: 0,
            limit: 100,
        }
    }

    /// Constructs a new `Updater` and starts the threads.
    pub fn start(self, mut dispatcher: Dispatcher) {
        let bot = Arc::clone(&self.bot);

        let start = self.then(Ok).for_each(move |update| {
            match update {
                Ok(u) => dispatcher.handle(&bot, u),
                Err(e) => eprintln!("Error: {}", e.as_fail()),
            }
            future::ok(())
        });

        tokio::run(start);
    }
}

impl Stream for Updater {
    type Item = Update;

    type Error = FailureError;

    fn poll(&mut self) -> Poll<Option<Update>, FailureError> {
        loop {
            let update = self.updates.pop();

            if let Some(u) = update {
                self.offset = (u.update_id + 1) as usize;
                return Ok(Async::Ready(Some(u)));
            } else if let Some(ref mut f) = self.update_future {
                let mut updates = try_ready!(f.poll());
                self.update_future = None;

                if updates.is_empty() {
                    continue;
                }

                updates.reverse();
                self.updates.append(&mut updates);
                let u = self.updates.pop().unwrap();
                self.offset = (u.update_id + 1) as usize;

                return Ok(Async::Ready(Some(u)));
            } else {
                let updates = self.bot.get_updates(self.offset, self.limit, self.timeout);
                self.update_future = Some(Box::new(updates));
            }
        }
    }
}

impl Debug for Updater {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let update_future = match &self.update_future {
            Some(_) => "Some(Box<dyn Future<Item = Vec<Update>, Error = FailureError> + Send>)",
            None => "None",
        };

        f.debug_struct("Updater")
            .field("timeout", &self.timeout)
            .field("offset", &self.offset)
            .field("limit", &self.limit)
            .field("update_future", &update_future)
            .field("updates", &self.updates)
            .field("bot", &self.bot)
            .finish()
    }
}

impl UpdaterBuilder {
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
            timeout,
            offset,
            limit,
        } = self;

        let bot = Arc::new(Bot::new(token).unwrap());
        let updates = Vec::new();
        let update_future = None;

        Updater {
            timeout,
            offset,
            limit,
            updates,
            bot,
            update_future,
        }
    }
}
