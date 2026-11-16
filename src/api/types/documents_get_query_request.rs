pub use crate::prelude::*;

/// Query parameters for Get Documentation From Knowledge Base
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DocumentsGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

impl DocumentsGetQueryRequest {
    pub fn builder() -> DocumentsGetQueryRequestBuilder {
        <DocumentsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentsGetQueryRequestBuilder {
    agent_id: Option<String>,
}

impl DocumentsGetQueryRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DocumentsGetQueryRequest`].
    pub fn build(self) -> Result<DocumentsGetQueryRequest, BuildError> {
        Ok(DocumentsGetQueryRequest {
            agent_id: self.agent_id,
        })
    }
}
