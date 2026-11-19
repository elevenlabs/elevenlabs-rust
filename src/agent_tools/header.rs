//! Webhook request header values and auth connections (kept in wire format).

use serde_json::{json, Value};

/// The value of a webhook request header.
#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    model: Value,
}

impl Header {
    /// A fixed value.
    pub fn value(text: impl Into<String>) -> Header {
        Header {
            model: Value::String(text.into()),
        }
    }

    /// A secret from the workspace's secret store, so the value never appears in the config.
    pub fn secret(secret_id: impl Into<String>) -> Header {
        Header {
            model: json!({ "secret_id": secret_id.into() }),
        }
    }

    /// The value of a dynamic variable when the tool runs.
    pub fn variable(name: impl Into<String>) -> Header {
        Header {
            model: json!({ "variable_name": name.into() }),
        }
    }

    /// A workspace environment variable, by label.
    pub fn env_var(label: impl Into<String>) -> Header {
        Header {
            model: json!({ "env_var_label": label.into() }),
        }
    }

    /// The header value in API wire format.
    pub fn to_wire(&self) -> Value {
        self.model.clone()
    }
}

/// The auth connection a webhook tool authenticates with.
#[derive(Debug, Clone, PartialEq)]
pub struct AuthConnection {
    model: Value,
}

impl AuthConnection {
    /// An auth connection from the workspace's store, by id.
    pub fn id(auth_connection_id: impl Into<String>) -> AuthConnection {
        AuthConnection {
            model: json!({ "auth_connection_id": auth_connection_id.into() }),
        }
    }

    /// An environment variable of type auth connection, by label, resolved for the current
    /// environment.
    pub fn env_var(label: impl Into<String>) -> AuthConnection {
        AuthConnection {
            model: json!({ "env_var_label": label.into() }),
        }
    }

    /// The auth connection in API wire format.
    pub fn to_wire(&self) -> Value {
        self.model.clone()
    }
}
