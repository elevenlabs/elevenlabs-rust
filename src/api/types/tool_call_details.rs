pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ToolCallDetails {
    ConversationHistoryTranscriptToolCallWebhookDetails(
        ConversationHistoryTranscriptToolCallWebhookDetails,
    ),
    ConversationHistoryTranscriptToolCallClientDetails(
        ConversationHistoryTranscriptToolCallClientDetails,
    ),
    ConversationHistoryTranscriptToolCallMcpDetails(
        ConversationHistoryTranscriptToolCallMcpDetails,
    ),
    ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput(
        ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput,
    ),
}

impl<'de> serde::Deserialize<'de> for ToolCallDetails {
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
                ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput,
            >(value.clone())
            {
                return Ok(
                    Self::ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput(
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
                ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput,
            >(value.clone())
            {
                return Ok(
                    Self::ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput(
                        parsed,
                    ),
                );
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum ToolCallDetails",
        ))
    }
}

impl ToolCallDetails {
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

    pub fn is_conversation_history_transcript_tool_call_api_integration_webhook_details_output(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput(_)
        )
    }

    pub fn as_conversation_history_transcript_tool_call_api_integration_webhook_details_output(
        &self,
    ) -> Option<&ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput> {
        match self {
            Self::ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput(
                value,
            ) => Some(value),
            _ => None,
        }
    }

    pub fn into_conversation_history_transcript_tool_call_api_integration_webhook_details_output(
        self,
    ) -> Option<ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput> {
        match self {
            Self::ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput(
                value,
            ) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ToolCallDetails {
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
            Self::ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput(
                value,
            ) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
