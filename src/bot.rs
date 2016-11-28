use std::io::Read;
use std::sync::mpsc;

use reqwest;
use reqwest::Client;
use json;

/// A struct which contains things associated with the bot.
pub struct Bot {
    id: i32,
    first_name: String,
    last_name: Option<String>,
    username: Option<String>,
    client: Client,
    pub bot_url: String,
}

impl Bot {
    /// Return a new bot struct.
    /// Panic! if something goes wrong.
    pub fn new(bot_url: String) -> Bot {
        let client = Client::new().unwrap();
        let mut rjson = Bot::get_me(&client, &bot_url);

        Bot {
            id: rjson["id"].as_i32().unwrap(),
            first_name: rjson["first_name"].take_string().unwrap(),
            last_name: rjson["last_name"].take_string(),
            username: rjson["username"].take_string(),
            client: client,
            bot_url: bot_url,
        }
    }

    pub fn handle_updates(&self, rx: mpsc::Receiver<String>) {
        loop {
            if let Ok(val) = rx.try_recv() {
                let rjson = json::parse(&val).unwrap();
                println!("{}", rjson["resut"]);
            } else {
                continue;
            }
        }
    }

    /// Gets the information about the bot.
    fn get_me(client: &Client, bot_url: &str) -> json::JsonValue {
        let path = ["getMe"];
        let url = ::construct_api_url(bot_url, &path);
        let mut resp = client.get(&url).send().unwrap();

        if resp.status().is_success() {
            let mut body = String::new();
            resp.read_to_string(&mut body).unwrap();
            let rjson = json::parse(&body).unwrap();
            rjson["result"].clone()
        } else {
            panic!("An error has occured");
        }
    }
}
