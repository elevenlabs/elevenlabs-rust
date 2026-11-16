pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AlertingWebhookNotifier {
    /// ID of the workspace webhook to deliver alert lifecycle notifications to.
    #[serde(default)]
    pub webhook_id: String,
}

impl AlertingWebhookNotifier {
    pub fn builder() -> AlertingWebhookNotifierBuilder {
        <AlertingWebhookNotifierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlertingWebhookNotifierBuilder {
    webhook_id: Option<String>,
}

impl AlertingWebhookNotifierBuilder {
    pub fn webhook_id(mut self, value: impl Into<String>) -> Self {
        self.webhook_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AlertingWebhookNotifier`].
    /// This method will fail if any of the following fields are not set:
    /// - [`webhook_id`](AlertingWebhookNotifierBuilder::webhook_id)
    pub fn build(self) -> Result<AlertingWebhookNotifier, BuildError> {
        Ok(AlertingWebhookNotifier {
            webhook_id: self
                .webhook_id
                .ok_or_else(|| BuildError::missing_field("webhook_id"))?,
        })
    }
}
