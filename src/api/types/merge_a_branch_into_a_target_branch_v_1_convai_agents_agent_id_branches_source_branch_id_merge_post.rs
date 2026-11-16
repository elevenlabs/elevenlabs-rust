pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePost {
    /// Whether to archive the source branch after merging
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_source_branch: Option<bool>,
    /// Force source branch changes onto the target, overriding timestamp-based conflict resolution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl MergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePost {
    pub fn builder(
    ) -> MergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePostBuilder
    {
        <MergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePostBuilder
{
    archive_source_branch: Option<bool>,
    force: Option<bool>,
}

impl MergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePostBuilder {
    pub fn archive_source_branch(mut self, value: bool) -> Self {
        self.archive_source_branch = Some(value);
        self
    }

    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePost`].
    pub fn build(
        self,
    ) -> Result<
        MergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePost,
        BuildError,
    > {
        Ok(
            MergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePost {
                archive_source_branch: self.archive_source_branch,
                force: self.force,
            },
        )
    }
}
