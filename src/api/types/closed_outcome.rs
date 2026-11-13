pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ClosedOutcome {
    pub reason: MergeProposalCloseReason,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub at: i64,
    #[serde(default)]
    pub by_user_id: Option<String>,
}

impl ClosedOutcome {
    pub fn builder() -> ClosedOutcomeBuilder {
        <ClosedOutcomeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClosedOutcomeBuilder {
    reason: Option<MergeProposalCloseReason>,
    at: Option<i64>,
    by_user_id: Option<String>,
}

impl ClosedOutcomeBuilder {
    pub fn reason(mut self, value: MergeProposalCloseReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn at(mut self, value: i64) -> Self {
        self.at = Some(value);
        self
    }

    pub fn by_user_id(mut self, value: impl Into<String>) -> Self {
        self.by_user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ClosedOutcome`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reason`](ClosedOutcomeBuilder::reason)
    /// - [`at`](ClosedOutcomeBuilder::at)
    pub fn build(self) -> Result<ClosedOutcome, BuildError> {
        Ok(ClosedOutcome {
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            at: self.at.ok_or_else(|| BuildError::missing_field("at"))?,
            by_user_id: self.by_user_id,
        })
    }
}
