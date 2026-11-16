pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationHistoryTranscriptToolCallWebhookDetails {
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_params: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_params: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl ConversationHistoryTranscriptToolCallWebhookDetails {
    pub fn builder() -> ConversationHistoryTranscriptToolCallWebhookDetailsBuilder {
        <ConversationHistoryTranscriptToolCallWebhookDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryTranscriptToolCallWebhookDetailsBuilder {
    method: Option<String>,
    url: Option<String>,
    headers: Option<std::collections::HashMap<String, String>>,
    path_params: Option<std::collections::HashMap<String, String>>,
    query_params: Option<std::collections::HashMap<String, String>>,
    body: Option<String>,
}

impl ConversationHistoryTranscriptToolCallWebhookDetailsBuilder {
    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn headers(mut self, value: std::collections::HashMap<String, String>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn path_params(mut self, value: std::collections::HashMap<String, String>) -> Self {
        self.path_params = Some(value);
        self
    }

    pub fn query_params(mut self, value: std::collections::HashMap<String, String>) -> Self {
        self.query_params = Some(value);
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryTranscriptToolCallWebhookDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](ConversationHistoryTranscriptToolCallWebhookDetailsBuilder::method)
    /// - [`url`](ConversationHistoryTranscriptToolCallWebhookDetailsBuilder::url)
    pub fn build(self) -> Result<ConversationHistoryTranscriptToolCallWebhookDetails, BuildError> {
        Ok(ConversationHistoryTranscriptToolCallWebhookDetails {
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            headers: self.headers,
            path_params: self.path_params,
            query_params: self.query_params,
            body: self.body,
        })
    }
}
