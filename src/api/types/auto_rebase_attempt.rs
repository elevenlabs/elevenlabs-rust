pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AutoRebaseAttempt {
    pub outcome: AutoRebaseOutcome,
    /// Unix seconds when the attempt ran
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub attempted_at: i64,
}

impl AutoRebaseAttempt {
    pub fn builder() -> AutoRebaseAttemptBuilder {
        <AutoRebaseAttemptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutoRebaseAttemptBuilder {
    outcome: Option<AutoRebaseOutcome>,
    attempted_at: Option<i64>,
}

impl AutoRebaseAttemptBuilder {
    pub fn outcome(mut self, value: AutoRebaseOutcome) -> Self {
        self.outcome = Some(value);
        self
    }

    pub fn attempted_at(mut self, value: i64) -> Self {
        self.attempted_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutoRebaseAttempt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`outcome`](AutoRebaseAttemptBuilder::outcome)
    /// - [`attempted_at`](AutoRebaseAttemptBuilder::attempted_at)
    pub fn build(self) -> Result<AutoRebaseAttempt, BuildError> {
        Ok(AutoRebaseAttempt {
            outcome: self
                .outcome
                .ok_or_else(|| BuildError::missing_field("outcome"))?,
            attempted_at: self
                .attempted_at
                .ok_or_else(|| BuildError::missing_field("attempted_at"))?,
        })
    }
}
