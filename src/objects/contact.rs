#[derive(Clone, Deserialize, Debug)]
pub struct Contact {
    pub phone_number: String;
    pub first_name: String;
    pub last_name: String;
    pub user_id: i64;
}
