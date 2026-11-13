pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ConversationHistoryTranscriptCommonModelOutputToolResultsItem {
    ConversationHistoryTranscriptOtherToolsResultCommonModel(
        ConversationHistoryTranscriptOtherToolsResultCommonModel,
    ),
    ConversationHistoryTranscriptSystemToolResultCommonModelOutput(
        ConversationHistoryTranscriptSystemToolResultCommonModelOutput,
    ),
    ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput(
        ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput,
    ),
    ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput(
        ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput,
    ),
}

impl<'de> serde::Deserialize<'de>
    for ConversationHistoryTranscriptCommonModelOutputToolResultsItem
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;

        let mut matched_literal = false;

        if value.get("type").and_then(serde_json::Value::as_str) == Some("system") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptSystemToolResultCommonModelOutput,
            >(value.clone())
            {
                return Ok(
                    Self::ConversationHistoryTranscriptSystemToolResultCommonModelOutput(parsed),
                );
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("api_integration_webhook")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput,
            >(value.clone())
            {
                return Ok(Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("workflow") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput,
            >(value.clone())
            {
                return Ok(
                    Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput(parsed),
                );
            }
        }

        if !matched_literal {
            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptOtherToolsResultCommonModel,
            >(value.clone())
            {
                return Ok(Self::ConversationHistoryTranscriptOtherToolsResultCommonModel(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptSystemToolResultCommonModelOutput,
            >(value.clone())
            {
                return Ok(
                    Self::ConversationHistoryTranscriptSystemToolResultCommonModelOutput(parsed),
                );
            }

            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput,
            >(value.clone())
            {
                return Ok(Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput,
            >(value.clone())
            {
                return Ok(
                    Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput(parsed),
                );
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum ConversationHistoryTranscriptCommonModelOutputToolResultsItem",
        ))
    }
}

impl ConversationHistoryTranscriptCommonModelOutputToolResultsItem {
    pub fn is_conversation_history_transcript_other_tools_result_common_model(&self) -> bool {
        matches!(
            self,
            Self::ConversationHistoryTranscriptOtherToolsResultCommonModel(_)
        )
    }

    pub fn as_conversation_history_transcript_other_tools_result_common_model(
        &self,
    ) -> Option<&ConversationHistoryTranscriptOtherToolsResultCommonModel> {
        match self {
            Self::ConversationHistoryTranscriptOtherToolsResultCommonModel(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_conversation_history_transcript_other_tools_result_common_model(
        self,
    ) -> Option<ConversationHistoryTranscriptOtherToolsResultCommonModel> {
        match self {
            Self::ConversationHistoryTranscriptOtherToolsResultCommonModel(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_conversation_history_transcript_system_tool_result_common_model_output(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::ConversationHistoryTranscriptSystemToolResultCommonModelOutput(_)
        )
    }

    pub fn as_conversation_history_transcript_system_tool_result_common_model_output(
        &self,
    ) -> Option<&ConversationHistoryTranscriptSystemToolResultCommonModelOutput> {
        match self {
            Self::ConversationHistoryTranscriptSystemToolResultCommonModelOutput(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_conversation_history_transcript_system_tool_result_common_model_output(
        self,
    ) -> Option<ConversationHistoryTranscriptSystemToolResultCommonModelOutput> {
        match self {
            Self::ConversationHistoryTranscriptSystemToolResultCommonModelOutput(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn is_conversation_history_transcript_api_integration_webhook_tools_result_common_model_output(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput(_)
        )
    }

    pub fn as_conversation_history_transcript_api_integration_webhook_tools_result_common_model_output(
        &self,
    ) -> Option<&ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput>
    {
        match self {
            Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_conversation_history_transcript_api_integration_webhook_tools_result_common_model_output(
        self,
    ) -> Option<ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput>
    {
        match self {
            Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_conversation_history_transcript_workflow_tools_result_common_model_output(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput(_)
        )
    }

    pub fn as_conversation_history_transcript_workflow_tools_result_common_model_output(
        &self,
    ) -> Option<&ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput> {
        match self {
            Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_conversation_history_transcript_workflow_tools_result_common_model_output(
        self,
    ) -> Option<ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput> {
        match self {
            Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for ConversationHistoryTranscriptCommonModelOutputToolResultsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversationHistoryTranscriptOtherToolsResultCommonModel(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ConversationHistoryTranscriptSystemToolResultCommonModelOutput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
