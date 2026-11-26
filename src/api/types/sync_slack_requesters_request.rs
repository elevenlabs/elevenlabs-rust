pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SyncSlackRequestersRequest {
    #[serde(default)]
    pub integration_connection_id: String,
}

impl SyncSlackRequestersRequest {
    pub fn builder() -> SyncSlackRequestersRequestBuilder {
        <SyncSlackRequestersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SyncSlackRequestersRequestBuilder {
    integration_connection_id: Option<String>,
}

impl SyncSlackRequestersRequestBuilder {
    pub fn integration_connection_id(mut self, value: impl Into<String>) -> Self {
        self.integration_connection_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SyncSlackRequestersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`integration_connection_id`](SyncSlackRequestersRequestBuilder::integration_connection_id)
    pub fn build(self) -> Result<SyncSlackRequestersRequest, BuildError> {
        Ok(SyncSlackRequestersRequest {
            integration_connection_id: self
                .integration_connection_id
                .ok_or_else(|| BuildError::missing_field("integration_connection_id"))?,
        })
    }
}
