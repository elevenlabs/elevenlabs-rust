pub use crate::prelude::*;

/// Query parameters for Get All Batch Calls For A Workspace.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BatchCallsListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_doc: Option<String>,
    /// Filter batch calls to a single agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

impl BatchCallsListQueryRequest {
    pub fn builder() -> BatchCallsListQueryRequestBuilder {
        <BatchCallsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchCallsListQueryRequestBuilder {
    limit: Option<i64>,
    last_doc: Option<String>,
    agent_id: Option<String>,
}

impl BatchCallsListQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn last_doc(mut self, value: impl Into<String>) -> Self {
        self.last_doc = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BatchCallsListQueryRequest`].
    pub fn build(self) -> Result<BatchCallsListQueryRequest, BuildError> {
        Ok(BatchCallsListQueryRequest {
            limit: self.limit,
            last_doc: self.last_doc,
            agent_id: self.agent_id,
        })
    }
}
