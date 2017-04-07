use serde_json;

use error::{Error, Result};

/// Extension for the `serde_json::Value`.
pub trait ValueExtension {
    fn as_optional_string(&self, field: &str) -> Option<String>;

    fn as_required_string(&self, field: &str) -> Result<String>;

    fn as_optional_i64(&self, field: &str) -> Option<i64>;

    fn as_required_i64(&self, field: &str) -> Result<i64>;

    fn as_optional_bool(&self, field: &str) -> Option<bool>;
}

impl ValueExtension for serde_json::Value {
    fn as_optional_string(&self, field: &str) -> Option<String> {
        self.get(field)
            .and_then(|v| v.as_str())
            .map(|s| s.to_owned())
    }

    fn as_required_string(&self, field: &str) -> Result<String> {
        self.get(field)
            .and_then(|v| v.as_str())
            .map(|s| s.to_owned())
            .ok_or(Error::JsonNotFound)
    }

    fn as_optional_i64(&self, field: &str) -> Option<i64> {
        self.get(field).and_then(|v| v.as_i64())
    }

    fn as_required_i64(&self, field: &str) -> Result<i64> {
        self.get(field)
            .and_then(|v| v.as_i64())
            .ok_or(Error::JsonNotFound)
    }

    fn as_optional_bool(&self, field: &str) -> Option<bool> {
        self.get(field).and_then(|v| v.as_bool())
    }
}
