const BASE_URL: &'static str = "https://api.telegram.org/bot";

pub struct Bot {
	token: String,
	bot_url: String,
}

impl Bot {
	fn new(token: String) -> Bot {
		let bot_url = [BASE_URL, &*token].concat();

		Bot {
			token: token,
			bot_url: bot_url,
		}
	}
}

fn construct_api_url(bot: Bot, path: &[&str]) -> String {
    let path = path.join("/");
    [bot.bot_url, path].join("/")
}
