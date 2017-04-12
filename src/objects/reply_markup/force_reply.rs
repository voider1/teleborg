/// Represents ForceReply from the Telegram bot API.
#[derive(Serialize, Debug)]
pub struct ForceReply {
    force_reply: bool,
    selective: Option<bool>,
}

impl ForceReply {
    /// Constructs a new `ForceReply`.
    pub fn new(force_reply: bool, selective: Option<bool>) -> ForceReply {
        ForceReply {
            force_reply: force_reply,
            selective: selective,
        }
    }
}
