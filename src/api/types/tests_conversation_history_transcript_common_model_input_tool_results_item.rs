pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ConversationHistoryTranscriptCommonModelInputToolResultsItem {
    ConversationHistoryTranscriptOtherToolsResultCommonModel(
        ConversationHistoryTranscriptOtherToolsResultCommonModel,
    ),
    ConversationHistoryTranscriptSystemToolResultCommonModelInput(
        ConversationHistoryTranscriptSystemToolResultCommonModelInput,
    ),
    ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput(
        ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput,
    ),
    ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput(
        ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput,
    ),
}

impl<'de> serde::Deserialize<'de> for ConversationHistoryTranscriptCommonModelInputToolResultsItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let matching_keys = |keys: &[&str]| {
            value
                .as_object()
                .map(|object| {
                    object
                        .keys()
                        .filter(|key| keys.contains(&key.as_str()))
                        .count()
                })
                .unwrap_or(0)
        };

        let mut matched_literal = false;

        if value.get("type").and_then(serde_json::Value::as_str) == Some("system") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptSystemToolResultCommonModelInput,
            >(value.clone())
            {
                let best = matching_keys(&[
                    "request_id",
                    "tool_name",
                    "tool_id",
                    "result_value",
                    "is_error",
                    "is_blocked",
                    "tool_has_been_called",
                    "tool_latency_secs",
                    "error_type",
                    "raw_error_message",
                    "dynamic_variable_updates",
                    "type",
                    "result",
                ]);
                let mut chosen = None;
                let score = matching_keys(&[
                    "request_id",
                    "tool_name",
                    "tool_id",
                    "result_value",
                    "is_error",
                    "is_blocked",
                    "tool_has_been_called",
                    "tool_latency_secs",
                    "error_type",
                    "raw_error_message",
                    "dynamic_variable_updates",
                    "type",
                ]);
                if score > best {
                    if let Ok(alternative) = serde_json::from_value::<
                        ConversationHistoryTranscriptOtherToolsResultCommonModel,
                    >(value.clone())
                    {
                        chosen = Some(
                            Self::ConversationHistoryTranscriptOtherToolsResultCommonModel(
                                alternative,
                            ),
                        );
                    }
                }
                return Ok(chosen.unwrap_or(
                    Self::ConversationHistoryTranscriptSystemToolResultCommonModelInput(parsed),
                ));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("api_integration_webhook")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput,
            >(value.clone())
            {
                let best = matching_keys(&[
                    "request_id",
                    "tool_name",
                    "tool_id",
                    "result_value",
                    "is_error",
                    "is_blocked",
                    "tool_has_been_called",
                    "tool_latency_secs",
                    "error_type",
                    "raw_error_message",
                    "dynamic_variable_updates",
                    "type",
                    "integration_id",
                    "credential_id",
                    "integration_connection_id",
                ]);
                let mut chosen = None;
                let score = matching_keys(&[
                    "request_id",
                    "tool_name",
                    "tool_id",
                    "result_value",
                    "is_error",
                    "is_blocked",
                    "tool_has_been_called",
                    "tool_latency_secs",
                    "error_type",
                    "raw_error_message",
                    "dynamic_variable_updates",
                    "type",
                ]);
                if score > best {
                    if let Ok(alternative) = serde_json::from_value::<
                        ConversationHistoryTranscriptOtherToolsResultCommonModel,
                    >(value.clone())
                    {
                        chosen = Some(
                            Self::ConversationHistoryTranscriptOtherToolsResultCommonModel(
                                alternative,
                            ),
                        );
                    }
                }
                return Ok(chosen.unwrap_or(Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput(parsed)));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("workflow") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput,
            >(value.clone())
            {
                let best = matching_keys(&[
                    "request_id",
                    "tool_name",
                    "tool_id",
                    "result_value",
                    "is_error",
                    "is_blocked",
                    "tool_has_been_called",
                    "tool_latency_secs",
                    "error_type",
                    "raw_error_message",
                    "dynamic_variable_updates",
                    "type",
                    "result",
                ]);
                let mut chosen = None;
                let score = matching_keys(&[
                    "request_id",
                    "tool_name",
                    "tool_id",
                    "result_value",
                    "is_error",
                    "is_blocked",
                    "tool_has_been_called",
                    "tool_latency_secs",
                    "error_type",
                    "raw_error_message",
                    "dynamic_variable_updates",
                    "type",
                ]);
                if score > best {
                    if let Ok(alternative) = serde_json::from_value::<
                        ConversationHistoryTranscriptOtherToolsResultCommonModel,
                    >(value.clone())
                    {
                        chosen = Some(
                            Self::ConversationHistoryTranscriptOtherToolsResultCommonModel(
                                alternative,
                            ),
                        );
                    }
                }
                return Ok(chosen.unwrap_or(
                    Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput(parsed),
                ));
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
                ConversationHistoryTranscriptSystemToolResultCommonModelInput,
            >(value.clone())
            {
                return Ok(
                    Self::ConversationHistoryTranscriptSystemToolResultCommonModelInput(parsed),
                );
            }

            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput,
            >(value.clone())
            {
                return Ok(Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<
                ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput,
            >(value.clone())
            {
                return Ok(
                    Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput(parsed),
                );
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum ConversationHistoryTranscriptCommonModelInputToolResultsItem",
        ))
    }
}

impl ConversationHistoryTranscriptCommonModelInputToolResultsItem {
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

    pub fn is_conversation_history_transcript_system_tool_result_common_model_input(&self) -> bool {
        matches!(
            self,
            Self::ConversationHistoryTranscriptSystemToolResultCommonModelInput(_)
        )
    }

    pub fn as_conversation_history_transcript_system_tool_result_common_model_input(
        &self,
    ) -> Option<&ConversationHistoryTranscriptSystemToolResultCommonModelInput> {
        match self {
            Self::ConversationHistoryTranscriptSystemToolResultCommonModelInput(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_conversation_history_transcript_system_tool_result_common_model_input(
        self,
    ) -> Option<ConversationHistoryTranscriptSystemToolResultCommonModelInput> {
        match self {
            Self::ConversationHistoryTranscriptSystemToolResultCommonModelInput(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn is_conversation_history_transcript_api_integration_webhook_tools_result_common_model_input(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput(_)
        )
    }

    pub fn as_conversation_history_transcript_api_integration_webhook_tools_result_common_model_input(
        &self,
    ) -> Option<&ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput>
    {
        match self {
            Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput(
                value,
            ) => Some(value),
            _ => None,
        }
    }

    pub fn into_conversation_history_transcript_api_integration_webhook_tools_result_common_model_input(
        self,
    ) -> Option<ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput> {
        match self {
            Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput(
                value,
            ) => Some(value),
            _ => None,
        }
    }

    pub fn is_conversation_history_transcript_workflow_tools_result_common_model_input(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput(_)
        )
    }

    pub fn as_conversation_history_transcript_workflow_tools_result_common_model_input(
        &self,
    ) -> Option<&ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput> {
        match self {
            Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_conversation_history_transcript_workflow_tools_result_common_model_input(
        self,
    ) -> Option<ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput> {
        match self {
            Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for ConversationHistoryTranscriptCommonModelInputToolResultsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversationHistoryTranscriptOtherToolsResultCommonModel(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ConversationHistoryTranscriptSystemToolResultCommonModelInput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput(
                value,
            ) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
        }
    }
}
