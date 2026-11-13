pub use crate::prelude::*;

/// Query parameters for Create transcript
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpeechToTextConvertQueryRequest {
    /// A single-use authentication token created via POST /v1/single-use-token/batch_scribe. This token can only be used once and expires after 15 minutes. Alternative to API key or bearer token authentication for frontend clients.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// When enable_logging is set to false zero retention mode will be used for the request. This will mean log and transcript storage features are unavailable for this request. Zero retention mode may only be used by enterprise customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_logging: Option<bool>,
}

impl SpeechToTextConvertQueryRequest {
    pub fn builder() -> SpeechToTextConvertQueryRequestBuilder {
        <SpeechToTextConvertQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeechToTextConvertQueryRequestBuilder {
    token: Option<String>,
    enable_logging: Option<bool>,
}

impl SpeechToTextConvertQueryRequestBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn enable_logging(mut self, value: bool) -> Self {
        self.enable_logging = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpeechToTextConvertQueryRequest`].
    pub fn build(self) -> Result<SpeechToTextConvertQueryRequest, BuildError> {
        Ok(SpeechToTextConvertQueryRequest {
            token: self.token,
            enable_logging: self.enable_logging,
        })
    }
}
