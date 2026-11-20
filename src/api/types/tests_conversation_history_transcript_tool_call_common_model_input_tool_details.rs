pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ConversationHistoryTranscriptToolCallCommonModelInputToolDetails {
    ConversationHistoryTranscriptToolCallWebhookDetails(
        ConversationHistoryTranscriptToolCallWebhookDetails,
    ),
    ConversationHistoryTranscriptToolCallClientDetails(
        ConversationHistoryTranscriptToolCallClientDetails,
    ),
    ConversationHistoryTranscriptToolCallMcpDetails(
        ConversationHistoryTranscriptToolCallMcpDetails,
    ),
    ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput(
        ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput,
    ),
}

impl<'de> serde::Deserialize<'de>
    for ConversationHistoryTranscriptToolCallCommonModelInputToolDetails
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;

        let mut matched_literal = false;

        if value.get("type").and_then(serde_json::Value::as_str) == Some("webhook") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptToolCallWebhookDetails,
            >(value.clone())
            {
                return Ok(Self::ConversationHistoryTranscriptToolCallWebhookDetails(
                    parsed,
                ));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("client") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptToolCallClientDetails,
            >(value.clone())
            {
                return Ok(Self::ConversationHistoryTranscriptToolCallClientDetails(
                    parsed,
                ));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("mcp") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptToolCallMcpDetails,
            >(value.clone())
            {
                return Ok(Self::ConversationHistoryTranscriptToolCallMcpDetails(
                    parsed,
                ));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("api_integration_webhook")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput,
            >(value.clone())
            {
                return Ok(
                    Self::ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput(
                        parsed,
                    ),
                );
            }
        }

        if !matched_literal {
            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptToolCallWebhookDetails,
            >(value.clone())
            {
                return Ok(Self::ConversationHistoryTranscriptToolCallWebhookDetails(
                    parsed,
                ));
            }

            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptToolCallClientDetails,
            >(value.clone())
            {
                return Ok(Self::ConversationHistoryTranscriptToolCallClientDetails(
                    parsed,
                ));
            }

            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptToolCallMcpDetails,
            >(value.clone())
            {
                return Ok(Self::ConversationHistoryTranscriptToolCallMcpDetails(
                    parsed,
                ));
            }

            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput,
            >(value.clone())
            {
                return Ok(
                    Self::ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput(
                        parsed,
                    ),
                );
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum ConversationHistoryTranscriptToolCallCommonModelInputToolDetails",
        ))
    }
}

impl ConversationHistoryTranscriptToolCallCommonModelInputToolDetails {
    pub fn is_conversation_history_transcript_tool_call_webhook_details(&self) -> bool {
        matches!(
            self,
            Self::ConversationHistoryTranscriptToolCallWebhookDetails(_)
        )
    }

    pub fn as_conversation_history_transcript_tool_call_webhook_details(
        &self,
    ) -> Option<&ConversationHistoryTranscriptToolCallWebhookDetails> {
        match self {
            Self::ConversationHistoryTranscriptToolCallWebhookDetails(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_conversation_history_transcript_tool_call_webhook_details(
        self,
    ) -> Option<ConversationHistoryTranscriptToolCallWebhookDetails> {
        match self {
            Self::ConversationHistoryTranscriptToolCallWebhookDetails(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_conversation_history_transcript_tool_call_client_details(&self) -> bool {
        matches!(
            self,
            Self::ConversationHistoryTranscriptToolCallClientDetails(_)
        )
    }

    pub fn as_conversation_history_transcript_tool_call_client_details(
        &self,
    ) -> Option<&ConversationHistoryTranscriptToolCallClientDetails> {
        match self {
            Self::ConversationHistoryTranscriptToolCallClientDetails(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_conversation_history_transcript_tool_call_client_details(
        self,
    ) -> Option<ConversationHistoryTranscriptToolCallClientDetails> {
        match self {
            Self::ConversationHistoryTranscriptToolCallClientDetails(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_conversation_history_transcript_tool_call_mcp_details(&self) -> bool {
        matches!(
            self,
            Self::ConversationHistoryTranscriptToolCallMcpDetails(_)
        )
    }

    pub fn as_conversation_history_transcript_tool_call_mcp_details(
        &self,
    ) -> Option<&ConversationHistoryTranscriptToolCallMcpDetails> {
        match self {
            Self::ConversationHistoryTranscriptToolCallMcpDetails(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_conversation_history_transcript_tool_call_mcp_details(
        self,
    ) -> Option<ConversationHistoryTranscriptToolCallMcpDetails> {
        match self {
            Self::ConversationHistoryTranscriptToolCallMcpDetails(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_conversation_history_transcript_tool_call_api_integration_webhook_details_input(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput(_)
        )
    }

    pub fn as_conversation_history_transcript_tool_call_api_integration_webhook_details_input(
        &self,
    ) -> Option<&ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput> {
        match self {
            Self::ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_conversation_history_transcript_tool_call_api_integration_webhook_details_input(
        self,
    ) -> Option<ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput> {
        match self {
            Self::ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for ConversationHistoryTranscriptToolCallCommonModelInputToolDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversationHistoryTranscriptToolCallWebhookDetails(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ConversationHistoryTranscriptToolCallClientDetails(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ConversationHistoryTranscriptToolCallMcpDetails(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsInput(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
        }
    }
}
