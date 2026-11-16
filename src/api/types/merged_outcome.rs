pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MergedOutcome {
    #[serde(default)]
    pub version_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub at: i64,
    #[serde(default)]
    pub by_user_id: Option<String>,
}

impl MergedOutcome {
    pub fn builder() -> MergedOutcomeBuilder {
        <MergedOutcomeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MergedOutcomeBuilder {
    version_id: Option<String>,
    at: Option<i64>,
    by_user_id: Option<String>,
}

impl MergedOutcomeBuilder {
    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`MergedOutcome`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version_id`](MergedOutcomeBuilder::version_id)
    /// - [`at`](MergedOutcomeBuilder::at)
    pub fn build(self) -> Result<MergedOutcome, BuildError> {
        Ok(MergedOutcome {
            version_id: self
                .version_id
                .ok_or_else(|| BuildError::missing_field("version_id"))?,
            at: self.at.ok_or_else(|| BuildError::missing_field("at"))?,
            by_user_id: self.by_user_id,
        })
    }
}
