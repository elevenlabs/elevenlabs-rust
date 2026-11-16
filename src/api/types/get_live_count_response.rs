pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetLiveCountResponse {
    /// The number of active ongoing conversations.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub count: i64,
}

impl GetLiveCountResponse {
    pub fn builder() -> GetLiveCountResponseBuilder {
        <GetLiveCountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLiveCountResponseBuilder {
    count: Option<i64>,
}

impl GetLiveCountResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetLiveCountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](GetLiveCountResponseBuilder::count)
    pub fn build(self) -> Result<GetLiveCountResponse, BuildError> {
        Ok(GetLiveCountResponse {
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
        })
    }
}
