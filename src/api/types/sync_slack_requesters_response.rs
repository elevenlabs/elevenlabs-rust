pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SyncSlackRequestersResponse {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub created_count: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub existing_count: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub skipped_count: i64,
}

impl SyncSlackRequestersResponse {
    pub fn builder() -> SyncSlackRequestersResponseBuilder {
        <SyncSlackRequestersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SyncSlackRequestersResponseBuilder {
    created_count: Option<i64>,
    existing_count: Option<i64>,
    skipped_count: Option<i64>,
}

impl SyncSlackRequestersResponseBuilder {
    pub fn created_count(mut self, value: i64) -> Self {
        self.created_count = Some(value);
        self
    }

    pub fn existing_count(mut self, value: i64) -> Self {
        self.existing_count = Some(value);
        self
    }

    pub fn skipped_count(mut self, value: i64) -> Self {
        self.skipped_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SyncSlackRequestersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_count`](SyncSlackRequestersResponseBuilder::created_count)
    /// - [`existing_count`](SyncSlackRequestersResponseBuilder::existing_count)
    /// - [`skipped_count`](SyncSlackRequestersResponseBuilder::skipped_count)
    pub fn build(self) -> Result<SyncSlackRequestersResponse, BuildError> {
        Ok(SyncSlackRequestersResponse {
            created_count: self
                .created_count
                .ok_or_else(|| BuildError::missing_field("created_count"))?,
            existing_count: self
                .existing_count
                .ok_or_else(|| BuildError::missing_field("existing_count"))?,
            skipped_count: self
                .skipped_count
                .ok_or_else(|| BuildError::missing_field("skipped_count"))?,
        })
    }
}
