use std::option::Option;

#[derive(Serialize, Debug)]
pub struct Markup {
	pub inline_keyboard: Vec<Vec<Button>>,
}

#[derive(Serialize, Debug)]
pub struct Button {
	text: String,
	url: Option<String>,
	callback_data: Option<String>,
	switch_inline_query: Option<String>,
	switch_inline_query_current_chat: Option<String>,
}

impl Markup {
	pub fn new(inline_keyboard: Vec<Vec<Button>>) -> Markup {
		Markup {
			inline_keyboard: inline_keyboard,
		}
	}
}

impl Button {
	pub fn new(text: String, 
				url: Option<String>, 
				callback_data: Option<String>, 
				switch_inline_query: Option<String>, 
				switch_inline_query_current_chat: Option<String>) -> Button {
		Button {
			text: text,
			url: Some(url.unwrap_or("".to_string())),
			callback_data: Some(callback_data.unwrap_or("".to_string())),
			switch_inline_query: Some(switch_inline_query.unwrap_or("".to_string())),
			switch_inline_query_current_chat: Some(switch_inline_query_current_chat.unwrap_or("".to_string())),
		}
	}
}