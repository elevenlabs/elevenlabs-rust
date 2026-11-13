pub use crate::prelude::*;

/// Query parameters for Delete agent
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeleteQueryRequest {
    /// If set to true, the agent is deleted even if other agents use it as a subagent, and it is removed from those agents and their branches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl DeleteQueryRequest {
    pub fn builder() -> DeleteQueryRequestBuilder {
        <DeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteQueryRequestBuilder {
    force: Option<bool>,
}

impl DeleteQueryRequestBuilder {
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteQueryRequest`].
    pub fn build(self) -> Result<DeleteQueryRequest, BuildError> {
        Ok(DeleteQueryRequest { force: self.force })
    }
}
