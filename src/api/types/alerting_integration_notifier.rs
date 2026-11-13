pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AlertingIntegrationNotifier {
    /// Integration to deliver to. The server treats an omitted value as ``pagerduty``.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<AlertingIntegrationNotifierIntegrationType>,
    /// ID of the workspace integration connection to deliver alert lifecycle notifications to. The connection's integration must have the monitoring capability and match ``integration_type``.
    #[serde(default)]
    pub connection_id: String,
    /// ID of the Slack channel to post alert notifications to, e.g. ``C0123456789``. Required when ``integration_type`` is ``slack``. The Slack app must be a member of the channel and have the ``chat:write`` scope, or ``chat:write.public`` for public channels it has not joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
}

impl AlertingIntegrationNotifier {
    pub fn builder() -> AlertingIntegrationNotifierBuilder {
        <AlertingIntegrationNotifierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlertingIntegrationNotifierBuilder {
    integration_type: Option<AlertingIntegrationNotifierIntegrationType>,
    connection_id: Option<String>,
    channel_id: Option<String>,
}

impl AlertingIntegrationNotifierBuilder {
    pub fn integration_type(mut self, value: AlertingIntegrationNotifierIntegrationType) -> Self {
        self.integration_type = Some(value);
        self
    }

    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AlertingIntegrationNotifier`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](AlertingIntegrationNotifierBuilder::connection_id)
    pub fn build(self) -> Result<AlertingIntegrationNotifier, BuildError> {
        Ok(AlertingIntegrationNotifier {
            integration_type: self.integration_type,
            connection_id: self
                .connection_id
                .ok_or_else(|| BuildError::missing_field("connection_id"))?,
            channel_id: self.channel_id,
        })
    }
}
