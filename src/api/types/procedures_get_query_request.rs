pub use crate::prelude::*;

/// Query parameters for Get Procedure
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProceduresGetQueryRequest {
    /// The version ID to retrieve. If omitted, returns the version at branch HEAD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// The agent version ID to retrieve the procedure for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version_id: Option<String>,
}

impl ProceduresGetQueryRequest {
    pub fn builder() -> ProceduresGetQueryRequestBuilder {
        <ProceduresGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProceduresGetQueryRequestBuilder {
    version_id: Option<String>,
    agent_version_id: Option<String>,
}

impl ProceduresGetQueryRequestBuilder {
    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn agent_version_id(mut self, value: impl Into<String>) -> Self {
        self.agent_version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProceduresGetQueryRequest`].
    pub fn build(self) -> Result<ProceduresGetQueryRequest, BuildError> {
        Ok(ProceduresGetQueryRequest {
            version_id: self.version_id,
            agent_version_id: self.agent_version_id,
        })
    }
}
