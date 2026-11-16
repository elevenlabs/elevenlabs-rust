pub use crate::prelude::*;

/// Deliver the result to specific configured flows webhooks.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookTargetIds {
    /// The IDs of the workspace flows webhooks to deliver the result to. Each must be one of the workspace's configured flows webhooks.
    #[serde(default)]
    pub ids: Vec<String>,
}

impl WebhookTargetIds {
    pub fn builder() -> WebhookTargetIdsBuilder {
        <WebhookTargetIdsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookTargetIdsBuilder {
    ids: Option<Vec<String>>,
}

impl WebhookTargetIdsBuilder {
    pub fn ids(mut self, value: Vec<String>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhookTargetIds`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ids`](WebhookTargetIdsBuilder::ids)
    pub fn build(self) -> Result<WebhookTargetIds, BuildError> {
        Ok(WebhookTargetIds {
            ids: self.ids.ok_or_else(|| BuildError::missing_field("ids"))?,
        })
    }
}
