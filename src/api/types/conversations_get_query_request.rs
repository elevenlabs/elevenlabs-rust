pub use crate::prelude::*;

/// Query parameters for Get Conversation Details
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationsGetQueryRequest {
    /// Response format. Defaults to 'json'. Set to 'opentelemetry' for an OTLP-compatible trace payload using the same structure as the post-call webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<GetFormat>,
}

impl ConversationsGetQueryRequest {
    pub fn builder() -> ConversationsGetQueryRequestBuilder {
        <ConversationsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsGetQueryRequestBuilder {
    format: Option<GetFormat>,
}

impl ConversationsGetQueryRequestBuilder {
    pub fn format(mut self, value: GetFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsGetQueryRequest`].
    pub fn build(self) -> Result<ConversationsGetQueryRequest, BuildError> {
        Ok(ConversationsGetQueryRequest {
            format: self.format,
        })
    }
}
