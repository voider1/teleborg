use super::Method;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// If you'd like to make sure that the Webhook request comes from Telegram, we recommend using a secret path in the URL, e.g. https://www.example.com/<token>. Since nobody else knows your bot‘s token, you can be pretty sure it’s us.
#[derive(Debug, TypedBuilder, Serialize)]
pub struct SetWebhook {
    /// HTTPS url to send updates to. Use an empty string to remove webhook integration
    pub url: String,
    #[builder(default)]
    /// certificate file to send with multipart
    pub certificate: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to 40. Use lower values to limit the load on your bot‘s server, and higher values to increase your bot’s throughput.
    pub max_connections: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List the types of updates you want your bot to receive. For example, specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all updates regardless of type (default). If not specified, the previous setting will be used.Please note that this parameter doesn't affect updates created before the call to the setWebhook, so unwanted updates may be received for a short period of time.
    pub allowed_updates: Option<Vec<String>>,
}

impl_method!(SetWebhook, bool, certificate);
