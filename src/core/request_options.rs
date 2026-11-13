use std::collections::HashMap;

/// Options for customizing individual requests
#[derive(Debug, Clone, Default)]
pub struct RequestOptions {
    /// API key for authentication (overrides client-level API key)
    pub api_key: Option<String>,
    /// Bearer token for authentication (overrides client-level token)
    pub token: Option<String>,
    /// Maximum number of retry attempts for failed requests (overrides client-level max retries)
    pub max_retries: Option<u32>,
    /// Request timeout in seconds (overrides client-level timeout)
    pub timeout_seconds: Option<u64>,
    /// Additional headers to include in the request
    pub additional_headers: HashMap<String, String>,
    /// Additional query parameters to include in the request
    pub additional_query_params: HashMap<String, String>,
    /// Additional JSON body properties, keyed by wire name. Merged over the serialized body;
    /// a key that matches a declared field replaces its value.
    pub additional_body_params: HashMap<String, serde_json::Value>,
}

impl RequestOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Overrides the client-level retry budget for this call.
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = Some(retries);
        self
    }

    /// Overrides the client-level timeout for this call.
    pub fn timeout_seconds(mut self, timeout: u64) -> Self {
        self.timeout_seconds = Some(timeout);
        self
    }

    pub fn additional_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.additional_headers.insert(key.into(), value.into());
        self
    }

    pub fn additional_query_param(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.additional_query_params
            .insert(key.into(), value.into());
        self
    }

    pub fn additional_body_param(
        mut self,
        key: impl Into<String>,
        value: impl Into<serde_json::Value>,
    ) -> Self {
        self.additional_body_params.insert(key.into(), value.into());
        self
    }

    /// The serialized `body` with `additional_body_params` shallow-merged over it, or `None`
    /// when there is nothing to merge or the body does not serialize, so the caller sends the
    /// body as-is. A null body becomes the extras; an array or primitive body is unchanged.
    pub fn merge_into_body<T: serde::Serialize + ?Sized>(
        &self,
        body: &T,
    ) -> Option<serde_json::Value> {
        if self.additional_body_params.is_empty() {
            return None;
        }
        let mut value = serde_json::to_value(body).ok()?;
        if value.is_null() {
            value = serde_json::Value::Object(serde_json::Map::new());
        }
        if let Some(map) = value.as_object_mut() {
            for (key, extra) in &self.additional_body_params {
                map.insert(key.clone(), extra.clone());
            }
        }
        Some(value)
    }
}
