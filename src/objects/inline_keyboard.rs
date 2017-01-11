use std::option::Option;

#[derive(Serialize, Debug)]
pub struct Markup {
	pub inline_keyboard: Vec<Vec<Button>>,
}

#[derive(Serialize, Debug)]
pub struct Button {
	text: String,
	url: Option<String>,
}

impl Markup {
	pub fn new(inline_keyboard: Vec<Button>) -> Markup {
		let mut buttons = Vec::<Vec<Button>>::new();
		buttons.push(inline_keyboard);
		Markup {
			inline_keyboard: buttons,
		}
	}
}

impl Button {
	pub fn new(text: String, url: Option<String>) -> Button {
		Button {
			text: text,
			url: url,
		}
	}
}