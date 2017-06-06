use objects::User;
use objects::Location;

/// Represents a result of an inline query that was chosen by the user and sent to their chat partner.
#[derive(Clone, Deserialize, Debug)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String,
}
