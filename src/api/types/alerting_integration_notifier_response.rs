pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AlertingIntegrationNotifierResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<AlertingIntegrationNotifierResponseIntegrationType>,
    #[serde(default)]
    pub connection_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
}

impl AlertingIntegrationNotifierResponse {
    pub fn builder() -> AlertingIntegrationNotifierResponseBuilder {
        <AlertingIntegrationNotifierResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlertingIntegrationNotifierResponseBuilder {
    integration_type: Option<AlertingIntegrationNotifierResponseIntegrationType>,
    connection_id: Option<String>,
    channel_id: Option<String>,
}

impl AlertingIntegrationNotifierResponseBuilder {
    pub fn integration_type(
        mut self,
        value: AlertingIntegrationNotifierResponseIntegrationType,
    ) -> Self {
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

    /// Consumes the builder and constructs a [`AlertingIntegrationNotifierResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](AlertingIntegrationNotifierResponseBuilder::connection_id)
    pub fn build(self) -> Result<AlertingIntegrationNotifierResponse, BuildError> {
        Ok(AlertingIntegrationNotifierResponse {
            integration_type: self.integration_type,
            connection_id: self
                .connection_id
                .ok_or_else(|| BuildError::missing_field("connection_id"))?,
            channel_id: self.channel_id,
        })
    }
}
