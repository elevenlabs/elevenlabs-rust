pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatch {
    /// New name for the branch. Must be unique within the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New description for the branch. Pass an empty string to clear it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the branch should be archived
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// The protection level for the branch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_status: Option<BranchProtectionStatus>,
    /// When true, rebase this branch automatically after each new version of the branch it is kept in sync with (`sync_branch_id`, main by default). Cannot be enabled on the main branch, or while this branch is more protected than a non-main branch it is kept in sync with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_rebase: Option<bool>,
    /// ID of the branch to keep this branch in sync with: its new versions are pulled in automatically while `auto_rebase` is on. Pass the main branch's ID to sync with main again. Must be the main branch or the active branch this branch was created from (`parent_branch_id`). At most 10 branches can be kept in sync with one another in a row. Cannot be set on the main branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_branch_id: Option<String>,
}

impl BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatch {
    pub fn builder() -> BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatchBuilder {
        <BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatchBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatchBuilder {
    name: Option<String>,
    description: Option<String>,
    is_archived: Option<bool>,
    protection_status: Option<BranchProtectionStatus>,
    auto_rebase: Option<bool>,
    sync_branch_id: Option<String>,
}

impl BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatchBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn is_archived(mut self, value: bool) -> Self {
        self.is_archived = Some(value);
        self
    }

    pub fn protection_status(mut self, value: BranchProtectionStatus) -> Self {
        self.protection_status = Some(value);
        self
    }

    pub fn auto_rebase(mut self, value: bool) -> Self {
        self.auto_rebase = Some(value);
        self
    }

    pub fn sync_branch_id(mut self, value: impl Into<String>) -> Self {
        self.sync_branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatch`].
    pub fn build(
        self,
    ) -> Result<BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatch, BuildError> {
        Ok(
            BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatch {
                name: self.name,
                description: self.description,
                is_archived: self.is_archived,
                protection_status: self.protection_status,
                auto_rebase: self.auto_rebase,
                sync_branch_id: self.sync_branch_id,
            },
        )
    }
}
