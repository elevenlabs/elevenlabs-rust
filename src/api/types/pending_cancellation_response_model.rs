pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PendingCancellationResponseModel {
    /// The timestamp of the cancellation.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub timestamp_seconds: i64,
}

impl PendingCancellationResponseModel {
    pub fn builder() -> PendingCancellationResponseModelBuilder {
        <PendingCancellationResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PendingCancellationResponseModelBuilder {
    timestamp_seconds: Option<i64>,
}

impl PendingCancellationResponseModelBuilder {
    pub fn timestamp_seconds(mut self, value: i64) -> Self {
        self.timestamp_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PendingCancellationResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`timestamp_seconds`](PendingCancellationResponseModelBuilder::timestamp_seconds)
    pub fn build(self) -> Result<PendingCancellationResponseModel, BuildError> {
        Ok(PendingCancellationResponseModel {
            timestamp_seconds: self
                .timestamp_seconds
                .ok_or_else(|| BuildError::missing_field("timestamp_seconds"))?,
        })
    }
}
