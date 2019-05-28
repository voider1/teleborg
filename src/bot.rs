use crate::{
    error::{Error, Result},
    methods::Method,
    types::User,
};
use failure::{ensure, Error as FailureError};
use futures::{future::err, Future};
use reqwest::{r#async::Client as AsyncClient, Client};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::{self, Value};
use std::ops::Deref;

const BASE_URL: &str = "https://api.telegram.org/bot";

/// A `Bot` which will do all the API calls.
///
/// The `Bot` will be given access to in a `Command` with which you can do all
/// the API interactions in your `Command`s.
#[derive(Debug)]
pub struct Bot {
    /// The bot's URL to which it will be making requests.
    pub bot_url: String,
    client: Client,
    async_client: AsyncClient,
    inner: User,
}

#[derive(Debug, Deserialize)]
struct TelegramResponse {
    ok: bool,
    error_code: Option<i32>,
    description: Option<String>,
    result: Option<Value>,
}

impl Bot {
    /// Constructs a new `Bot`.
    pub fn new(token: &str) -> Result<Self> {
        let client = Client::new();
        let async_client = AsyncClient::new();
        let bot_url = [BASE_URL, token].concat();
        let inner = Self::get_me(&client, &bot_url)?;

        Ok(Bot {
            bot_url,
            client,
            async_client,
            inner,
        })
    }

    fn get_me(client: &Client, bot_url: &str) -> Result<User> {
        let path = ["getMe"];
        let url = crate::construct_api_url(bot_url, &path);
        let resp = client.get(&url).send()?.json()?;
        let bot: User = Self::get_result(resp)?;

        Ok(bot)
    }

    /// The actual networking done for making API calls.
    pub fn call<M>(&self, m: M) -> Box<dyn Future<Item = M::Response, Error = FailureError> + Send>
    where
        M: Method + 'static + Send,
    {
        let url = [&self.bot_url, M::PATH].join("/");
        let body = match m.build(self.async_client.post(&url)) {
            Ok(o) => o,
            Err(e) => return Box::new(err(e)),
        };

        Box::new(
            body.send()
                .and_then(|mut res| res.json::<TelegramResponse>())
                .from_err()
                .and_then(Self::get_result),
        )
    }

    fn get_result<R>(resp: TelegramResponse) -> Result<R>
    where
        R: DeserializeOwned,
    {
        ensure!(
            resp.ok,
            Error::TelegramApiError(resp.description.unwrap(), resp.error_code.unwrap())
        );
        ensure!(resp.result.is_some(), Error::JsonNotFoundError);
        let result_val = resp.result.unwrap();
        let result: R = serde_json::from_value(result_val)?;
        Ok(result)
    }
}

impl Deref for Bot {
    type Target = User;

    fn deref(&self) -> &User {
        &self.inner
    }
}
