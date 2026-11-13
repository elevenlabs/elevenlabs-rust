pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ConversationHistoryTranscriptToolCallCommonModelInputToolDetails {
    #[serde(rename = "api_integration_webhook")]
    ApiIntegrationWebhook {
        #[serde(flatten)]
        data: ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput,
    },

    #[serde(rename = "client")]
    Client {
        #[serde(flatten)]
        data: ConversationHistoryTranscriptToolCallClientDetails,
    },

    #[serde(rename = "mcp")]
    Mcp {
        #[serde(flatten)]
        data: ConversationHistoryTranscriptToolCallMcpDetails,
    },

    #[serde(rename = "webhook")]
    Webhook {
        #[serde(flatten)]
        data: ConversationHistoryTranscriptToolCallWebhookDetails,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ConversationHistoryTranscriptToolCallCommonModelInputToolDetails {
    pub fn is_api_integration_webhook(&self) -> bool {
        matches!(self, Self::ApiIntegrationWebhook { .. })
    }

    pub fn as_api_integration_webhook(
        &self,
    ) -> Option<&ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput> {
        match self {
            Self::ApiIntegrationWebhook { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_api_integration_webhook(
        self,
    ) -> Option<ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput> {
        match self {
            Self::ApiIntegrationWebhook { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_client(&self) -> bool {
        matches!(self, Self::Client { .. })
    }

    pub fn as_client(&self) -> Option<&ConversationHistoryTranscriptToolCallClientDetails> {
        match self {
            Self::Client { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_client(self) -> Option<ConversationHistoryTranscriptToolCallClientDetails> {
        match self {
            Self::Client { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_mcp(&self) -> bool {
        matches!(self, Self::Mcp { .. })
    }

    pub fn as_mcp(&self) -> Option<&ConversationHistoryTranscriptToolCallMcpDetails> {
        match self {
            Self::Mcp { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_mcp(self) -> Option<ConversationHistoryTranscriptToolCallMcpDetails> {
        match self {
            Self::Mcp { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_webhook(&self) -> bool {
        matches!(self, Self::Webhook { .. })
    }

    pub fn as_webhook(&self) -> Option<&ConversationHistoryTranscriptToolCallWebhookDetails> {
        match self {
            Self::Webhook { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_webhook(self) -> Option<ConversationHistoryTranscriptToolCallWebhookDetails> {
        match self {
            Self::Webhook { data } => Some(data),
            _ => None,
        }
    }
}
