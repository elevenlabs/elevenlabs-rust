pub use crate::prelude::*;

/// Request for Rebase (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RebaseRequest {
    /// The ID of the branch to pull changes from. Defaults to the agent's main branch.
    #[serde(skip)]
    pub source_branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePost>,
}

impl RebaseRequest {
    pub fn builder() -> RebaseRequestBuilder {
        <RebaseRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RebaseRequestBuilder {
    source_branch_id: Option<String>,
    body: Option<BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePost>,
}

impl RebaseRequestBuilder {
    pub fn source_branch_id(mut self, value: impl Into<String>) -> Self {
        self.source_branch_id = Some(value.into());
        self
    }

    pub fn body(
        mut self,
        value: BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePost,
    ) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RebaseRequest`].
    pub fn build(self) -> Result<RebaseRequest, BuildError> {
        Ok(RebaseRequest {
            source_branch_id: self.source_branch_id,
            body: self.body,
        })
    }
}
