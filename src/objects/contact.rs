#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<i64>,
}

impl Contact {
	pub fn new(phone_number: String,
				first_name: String,
				last_name: Option<String>,
				user_id: Option<i64>) -> Contact {
		Contact {
			phone_number: phone_number.to_string(),
			first_name: first_name.to_string(),
			last_name: last_name,
			user_id: user_id,
		}
	}
}