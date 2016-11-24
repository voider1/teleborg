extern crate json;

use std::io::Read;

use reqwest;

/// A struct which contains things associated with the bot.
pub struct Bot {
    id: i32,
    first_name: String,
    last_name: String,
    username: String,
    bot_url: String,
}

impl Bot {
    /// Return a new bot struct.
    /// Panic! if something goes wrong.
	pub fn new(bot_url: String) -> Bot {
	    let mut resp = Bot::get_me(&bot_url);

	    if resp.status().is_success() {
            let mut body = String::new();
            resp.read_to_string(&mut body).unwrap();
            let mut rjson = json::parse(&body).unwrap();
            let rjson = &mut rjson["result"];
            let last_name: String;
            let username: String;

            if let Some(l_name) = rjson["last_name"].take_string() {
                last_name = l_name;
            } else {
                last_name = "".to_string();
            }

            if let Some(u_name) = rjson["username"].take_string() {
                username = u_name;
            } else {
                username = "".to_string();
            }

            Bot {
                id: rjson["id"].as_i32().unwrap(),
                first_name: rjson["first_name"].take_string().unwrap(),
                last_name: last_name,
                username: username,
                bot_url: bot_url,
            }
        } else {
            panic!("An error has occured");
        }
	}

    /// Gets the information about the bot.
	fn get_me(bot_url: &str) -> reqwest::Response {
	    let path = ["getMe"];
	    let url = construct_api_url(bot_url, &path);
	    reqwest::get(&url).unwrap()
    }
}

/// Construct an API URL with the base bot URL and an
/// array of strings which will construct the path.
fn construct_api_url(bot_url: &str, path: &[&str]) -> String {
    format!("{}/{}", bot_url, path.join("/"))
}
