use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("BadRequestError (HTTP 400): {0:?}")]
    BadRequestError(crate::api::types::DownloadBadRequestResponse),
    #[error("UnprocessableEntityError (HTTP 422): {0:?}")]
    UnprocessableEntityError(crate::api::types::HttpValidationError),
    #[error("HTTP error {status}: {message}")]
    Http { status: u16, message: String },
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Request executor error: {0}")]
    Executor(Box<dyn std::error::Error + Send + Sync>),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Invalid header value")]
    InvalidHeader,
    #[error("Could not clone request for retry")]
    RequestClone,
    #[error("SSE stream terminated")]
    StreamTerminated,
    #[error("SSE stream timed out waiting for next event")]
    StreamTimeout,
    #[error("SSE parse error: {0}")]
    SseParseError(String),
    #[error("WebSocket error: {0}")]
    WebSocketError(String),
    #[error("Failed to decode response body (HTTP {status}): {message}")]
    DecodeError { status: u16, message: String },
}

impl ApiError {
    pub fn from_response(status_code: u16, body: Option<&str>) -> Self {
        match status_code {
            400 => {
                if let Some(parsed) = body.and_then(|body_str| {
                    serde_json::from_str::<crate::api::types::DownloadBadRequestResponse>(body_str)
                        .ok()
                        .filter(|parsed| keeps_response_fields(parsed, body_str))
                }) {
                    Self::BadRequestError(parsed)
                } else {
                    Self::Http {
                        status: status_code,
                        message: body.unwrap_or("Unknown error").to_string(),
                    }
                }
            }
            422 => {
                if let Some(parsed) = body.and_then(|body_str| {
                    serde_json::from_str::<crate::api::types::HttpValidationError>(body_str)
                        .ok()
                        .filter(|parsed| keeps_response_fields(parsed, body_str))
                }) {
                    Self::UnprocessableEntityError(parsed)
                } else {
                    Self::Http {
                        status: status_code,
                        message: body.unwrap_or("Unknown error").to_string(),
                    }
                }
            }
            _ => Self::Http {
                status: status_code,
                message: body.unwrap_or("Unknown error").to_string(),
            },
        }
    }
}

fn keeps_response_fields<T: serde::Serialize>(parsed: &T, body: &str) -> bool {
    let (Ok(serde_json::Value::Object(sent)), Ok(serde_json::Value::Object(kept))) = (
        serde_json::from_str::<serde_json::Value>(body),
        serde_json::to_value(parsed),
    ) else {
        return true;
    };
    let mut sent_fields = sent.iter().filter(|(_, value)| !value.is_null()).peekable();
    sent_fields.peek().is_none() || sent_fields.any(|(name, _)| kept.contains_key(name))
}

/// Error returned when a required field was not set on a builder.
#[derive(Debug)]
pub struct BuildError {
    field: &'static str,
}

impl BuildError {
    pub fn missing_field(field: &'static str) -> Self {
        Self { field }
    }
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}` was not set but is required", self.field)
    }
}

impl std::error::Error for BuildError {}
