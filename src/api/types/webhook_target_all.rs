pub use crate::prelude::*;

/// Deliver the result to all of the workspace's configured flows webhooks.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookTargetAll {}

impl WebhookTargetAll {
    pub fn builder() -> WebhookTargetAllBuilder {
        <WebhookTargetAllBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookTargetAllBuilder {}

impl WebhookTargetAllBuilder {
    /// Consumes the builder and constructs a [`WebhookTargetAll`].
    pub fn build(self) -> Result<WebhookTargetAll, BuildError> {
        Ok(WebhookTargetAll {})
    }
}
