pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_connection_id: Option<String>,
    pub webhook_details: ConversationHistoryTranscriptToolCallWebhookDetails,
}

impl ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput {
    pub fn builder() -> ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInputBuilder
    {
        <ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInputBuilder {
    integration_id: Option<String>,
    credential_id: Option<String>,
    integration_connection_id: Option<String>,
    webhook_details: Option<ConversationHistoryTranscriptToolCallWebhookDetails>,
}

impl ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInputBuilder {
    pub fn integration_id(mut self, value: impl Into<String>) -> Self {
        self.integration_id = Some(value.into());
        self
    }

    pub fn credential_id(mut self, value: impl Into<String>) -> Self {
        self.credential_id = Some(value.into());
        self
    }

    pub fn integration_connection_id(mut self, value: impl Into<String>) -> Self {
        self.integration_connection_id = Some(value.into());
        self
    }

    pub fn webhook_details(
        mut self,
        value: ConversationHistoryTranscriptToolCallWebhookDetails,
    ) -> Self {
        self.webhook_details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`webhook_details`](ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInputBuilder::webhook_details)
    pub fn build(
        self,
    ) -> Result<ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput, BuildError>
    {
        Ok(
            ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput {
                integration_id: self.integration_id,
                credential_id: self.credential_id,
                integration_connection_id: self.integration_connection_id,
                webhook_details: self
                    .webhook_details
                    .ok_or_else(|| BuildError::missing_field("webhook_details"))?,
            },
        )
    }
}
