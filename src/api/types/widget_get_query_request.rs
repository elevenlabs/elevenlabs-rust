pub use crate::prelude::*;

/// Query parameters for Get Agent Widget Config
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WidgetGetQueryRequest {
    /// An expiring token that enables a websocket conversation to start. These can be generated for an agent using the /v1/convai/conversation/get_signed_url endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_signature: Option<String>,
}

impl WidgetGetQueryRequest {
    pub fn builder() -> WidgetGetQueryRequestBuilder {
        <WidgetGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WidgetGetQueryRequestBuilder {
    conversation_signature: Option<String>,
}

impl WidgetGetQueryRequestBuilder {
    pub fn conversation_signature(mut self, value: impl Into<String>) -> Self {
        self.conversation_signature = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WidgetGetQueryRequest`].
    pub fn build(self) -> Result<WidgetGetQueryRequest, BuildError> {
        Ok(WidgetGetQueryRequest {
            conversation_signature: self.conversation_signature,
        })
    }
}
