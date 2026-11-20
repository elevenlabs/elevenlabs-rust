pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput {
    pub r#type: ApiIntegrationWebhookToolConfigInputType,
    pub integration_id: String,
    pub credential_id: String,
    pub integration_connection_id: String,
    pub webhook_details: ConversationHistoryTranscriptToolCallWebhookDetails,
    pub function_details: Option<ConversationHistoryTranscriptToolCallIntegrationFunctionDetails>,
}

impl ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput {
    pub fn builder(
    ) -> ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutputBuilder {
        <ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutputBuilder {
    r#type: Option<ApiIntegrationWebhookToolConfigInputType>,
    integration_id: Option<String>,
    credential_id: Option<String>,
    integration_connection_id: Option<String>,
    webhook_details: Option<ConversationHistoryTranscriptToolCallWebhookDetails>,
    function_details: Option<ConversationHistoryTranscriptToolCallIntegrationFunctionDetails>,
}

impl ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutputBuilder {
    pub fn r#type(mut self, value: ApiIntegrationWebhookToolConfigInputType) -> Self {
        self.r#type = Some(value);
        self
    }

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

    pub fn function_details(
        mut self,
        value: ConversationHistoryTranscriptToolCallIntegrationFunctionDetails,
    ) -> Self {
        self.function_details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutputBuilder::type)
    /// - [`integration_id`](ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutputBuilder::integration_id)
    /// - [`credential_id`](ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutputBuilder::credential_id)
    /// - [`integration_connection_id`](ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutputBuilder::integration_connection_id)
    /// - [`webhook_details`](ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutputBuilder::webhook_details)
    pub fn build(
        self,
    ) -> Result<ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput, BuildError>
    {
        Ok(
            ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput {
                r#type: self
                    .r#type
                    .ok_or_else(|| BuildError::missing_field("r#type"))?,
                integration_id: self
                    .integration_id
                    .ok_or_else(|| BuildError::missing_field("integration_id"))?,
                credential_id: self
                    .credential_id
                    .ok_or_else(|| BuildError::missing_field("credential_id"))?,
                integration_connection_id: self
                    .integration_connection_id
                    .ok_or_else(|| BuildError::missing_field("integration_connection_id"))?,
                webhook_details: self
                    .webhook_details
                    .ok_or_else(|| BuildError::missing_field("webhook_details"))?,
                function_details: self.function_details,
            },
        )
    }
}
