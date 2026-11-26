pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BranchSyncStatus {
    /// ID of the branch this branch is kept in sync with: its new versions are pulled in automatically while `auto_rebase` is on. The main branch unless another was chosen. While the chosen branch is archived this still names it, and main is used in its place.
    #[serde(default)]
    pub branch_id: String,
    /// Latest automatic attempt to pull the synced branch's new versions in, run after each of them. Null when none has run since sync was turned on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_attempt: Option<AutoRebaseAttempt>,
}

impl BranchSyncStatus {
    pub fn builder() -> BranchSyncStatusBuilder {
        <BranchSyncStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BranchSyncStatusBuilder {
    branch_id: Option<String>,
    last_attempt: Option<AutoRebaseAttempt>,
}

impl BranchSyncStatusBuilder {
    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn last_attempt(mut self, value: AutoRebaseAttempt) -> Self {
        self.last_attempt = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BranchSyncStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`branch_id`](BranchSyncStatusBuilder::branch_id)
    pub fn build(self) -> Result<BranchSyncStatus, BuildError> {
        Ok(BranchSyncStatus {
            branch_id: self
                .branch_id
                .ok_or_else(|| BuildError::missing_field("branch_id"))?,
            last_attempt: self.last_attempt,
        })
    }
}
