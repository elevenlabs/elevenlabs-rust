pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpeechEngineConfig {
    /// The WebSocket URL for the transcript server
    #[serde(default)]
    pub ws_url: String,
    /// Headers to include in the WebSocket connection request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_headers:
        Option<std::collections::HashMap<String, SpeechEngineConfigRequestHeaders>>,
}

impl SpeechEngineConfig {
    pub fn builder() -> SpeechEngineConfigBuilder {
        <SpeechEngineConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeechEngineConfigBuilder {
    ws_url: Option<String>,
    request_headers: Option<std::collections::HashMap<String, SpeechEngineConfigRequestHeaders>>,
}

impl SpeechEngineConfigBuilder {
    pub fn ws_url(mut self, value: impl Into<String>) -> Self {
        self.ws_url = Some(value.into());
        self
    }

    pub fn request_headers(
        mut self,
        value: std::collections::HashMap<String, SpeechEngineConfigRequestHeaders>,
    ) -> Self {
        self.request_headers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpeechEngineConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ws_url`](SpeechEngineConfigBuilder::ws_url)
    pub fn build(self) -> Result<SpeechEngineConfig, BuildError> {
        Ok(SpeechEngineConfig {
            ws_url: self
                .ws_url
                .ok_or_else(|| BuildError::missing_field("ws_url"))?,
            request_headers: self.request_headers,
        })
    }
}
