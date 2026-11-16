pub use crate::prelude::*;

/// Request for Merge (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MergeRequest {
    /// The ID of the target branch to merge into.
    #[serde(skip)]
    pub target_branch_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body:
        Option<MergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePost>,
}

impl MergeRequest {
    pub fn builder() -> MergeRequestBuilder {
        <MergeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MergeRequestBuilder {
    target_branch_id: Option<String>,
    body: Option<MergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePost>,
}

impl MergeRequestBuilder {
    pub fn target_branch_id(mut self, value: impl Into<String>) -> Self {
        self.target_branch_id = Some(value.into());
        self
    }

    pub fn body(
        mut self,
        value: MergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePost,
    ) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MergeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`target_branch_id`](MergeRequestBuilder::target_branch_id)
    pub fn build(self) -> Result<MergeRequest, BuildError> {
        Ok(MergeRequest {
            target_branch_id: self
                .target_branch_id
                .ok_or_else(|| BuildError::missing_field("target_branch_id"))?,
            body: self.body,
        })
    }
}
