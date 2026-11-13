pub use crate::prelude::*;

/// Query parameters for List Procedures
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProceduresListQueryRequest {
    /// The agent version ID to retrieve the procedure for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version_id: Option<String>,
}

impl ProceduresListQueryRequest {
    pub fn builder() -> ProceduresListQueryRequestBuilder {
        <ProceduresListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProceduresListQueryRequestBuilder {
    agent_version_id: Option<String>,
}

impl ProceduresListQueryRequestBuilder {
    pub fn agent_version_id(mut self, value: impl Into<String>) -> Self {
        self.agent_version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProceduresListQueryRequest`].
    pub fn build(self) -> Result<ProceduresListQueryRequest, BuildError> {
        Ok(ProceduresListQueryRequest {
            agent_version_id: self.agent_version_id,
        })
    }
}
