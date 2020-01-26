use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// If you'd like to make sure that the Webhook request comes from Telegram, we recommend using a secret path in the URL, e.g. https://www.example.com/<token>. Since nobody else knows your bot‘s token, you can be pretty sure it’s us.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SetWebhook {
    /// HTTPS url to send updates to. Use an empty string to remove webhook integration
    pub url: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    /// Upload your public key certificate so that the root certificate in use can be
    pub certificate: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for
    pub max_connections: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List the types of updates you want your bot to receive. For example, specify
    pub allowed_updates: Option<Vec<String>>,
}

impl_method!(SetWebhook, bool, certificate);
