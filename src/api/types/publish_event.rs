pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum PublishEvent {
    UserAudioChunk(UserAudioChunk),
    Feedback(Feedback),
    UserMessage(UserMessage),
    UserActivity(UserActivity),
    MultimodalMessage(MultimodalMessage),
    ConversationPong(ConversationPong),
    ClientToolResult(ClientToolResult),
    McpToolApprovalResult(McpToolApprovalResult),
    ContextualUpdate(ContextualUpdate),
    ConversationInitiationClientData(ConversationInitiationClientData),
}

impl<'de> serde::Deserialize<'de> for PublishEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let has_any_key = |keys: &[&str]| {
            value
                .as_object()
                .map(|object| keys.iter().any(|key| object.contains_key(*key)))
                .unwrap_or(false)
        };

        let mut matched_literal = false;

        if value.get("type").and_then(serde_json::Value::as_str) == Some("feedback") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<Feedback>(value.clone()) {
                return Ok(Self::Feedback(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("user_message") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<UserMessage>(value.clone()) {
                return Ok(Self::UserMessage(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("user_activity") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<UserActivity>(value.clone()) {
                return Ok(Self::UserActivity(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("multimodal_message") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<MultimodalMessage>(value.clone()) {
                return Ok(Self::MultimodalMessage(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("pong") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<ConversationPong>(value.clone()) {
                return Ok(Self::ConversationPong(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("client_tool_result") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<ClientToolResult>(value.clone()) {
                return Ok(Self::ClientToolResult(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("mcp_tool_approval_result")
        {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<McpToolApprovalResult>(value.clone()) {
                return Ok(Self::McpToolApprovalResult(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("contextual_update") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<ContextualUpdate>(value.clone()) {
                return Ok(Self::ContextualUpdate(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str)
            == Some("conversation_initiation_client_data")
        {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<ConversationInitiationClientData>(value.clone())
            {
                return Ok(Self::ConversationInitiationClientData(parsed));
            }
        }

        if !matched_literal {
            if let Ok(parsed) = serde_json::from_value::<UserAudioChunk>(value.clone()) {
                return Ok(Self::UserAudioChunk(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<Feedback>(value.clone()) {
                return Ok(Self::Feedback(parsed));
            }

            if has_any_key(&["id", "type", "text", "user_identifier", "source_medium"]) {
                if let Ok(parsed) = serde_json::from_value::<UserMessage>(value.clone()) {
                    return Ok(Self::UserMessage(parsed));
                }
            }

            if has_any_key(&["type"]) {
                if let Ok(parsed) = serde_json::from_value::<UserActivity>(value.clone()) {
                    return Ok(Self::UserActivity(parsed));
                }
            }

            if has_any_key(&["type", "text", "file", "files"]) {
                if let Ok(parsed) = serde_json::from_value::<MultimodalMessage>(value.clone()) {
                    return Ok(Self::MultimodalMessage(parsed));
                }
            }

            if let Ok(parsed) = serde_json::from_value::<ConversationPong>(value.clone()) {
                return Ok(Self::ConversationPong(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<ClientToolResult>(value.clone()) {
                return Ok(Self::ClientToolResult(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<McpToolApprovalResult>(value.clone()) {
                return Ok(Self::McpToolApprovalResult(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<ContextualUpdate>(value.clone()) {
                return Ok(Self::ContextualUpdate(parsed));
            }

            if has_any_key(&[
                "conversation_config_override",
                "custom_llm_extra_body",
                "user_id",
                "source_info",
                "branch_id",
                "environment",
                "starting_workflow_node_id",
                "procedure_ids",
                "start_procedure_ids",
                "dynamic_variables",
                "type",
            ]) {
                if let Ok(parsed) =
                    serde_json::from_value::<ConversationInitiationClientData>(value.clone())
                {
                    return Ok(Self::ConversationInitiationClientData(parsed));
                }
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum PublishEvent",
        ))
    }
}

impl PublishEvent {
    pub fn is_user_audio_chunk(&self) -> bool {
        matches!(self, Self::UserAudioChunk(_))
    }

    pub fn as_user_audio_chunk(&self) -> Option<&UserAudioChunk> {
        match self {
            Self::UserAudioChunk(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_user_audio_chunk(self) -> Option<UserAudioChunk> {
        match self {
            Self::UserAudioChunk(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_feedback(&self) -> bool {
        matches!(self, Self::Feedback(_))
    }

    pub fn as_feedback(&self) -> Option<&Feedback> {
        match self {
            Self::Feedback(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_feedback(self) -> Option<Feedback> {
        match self {
            Self::Feedback(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_user_message(&self) -> bool {
        matches!(self, Self::UserMessage(_))
    }

    pub fn as_user_message(&self) -> Option<&UserMessage> {
        match self {
            Self::UserMessage(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_user_message(self) -> Option<UserMessage> {
        match self {
            Self::UserMessage(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_user_activity(&self) -> bool {
        matches!(self, Self::UserActivity(_))
    }

    pub fn as_user_activity(&self) -> Option<&UserActivity> {
        match self {
            Self::UserActivity(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_user_activity(self) -> Option<UserActivity> {
        match self {
            Self::UserActivity(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_multimodal_message(&self) -> bool {
        matches!(self, Self::MultimodalMessage(_))
    }

    pub fn as_multimodal_message(&self) -> Option<&MultimodalMessage> {
        match self {
            Self::MultimodalMessage(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_multimodal_message(self) -> Option<MultimodalMessage> {
        match self {
            Self::MultimodalMessage(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_conversation_pong(&self) -> bool {
        matches!(self, Self::ConversationPong(_))
    }

    pub fn as_conversation_pong(&self) -> Option<&ConversationPong> {
        match self {
            Self::ConversationPong(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_conversation_pong(self) -> Option<ConversationPong> {
        match self {
            Self::ConversationPong(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_client_tool_result(&self) -> bool {
        matches!(self, Self::ClientToolResult(_))
    }

    pub fn as_client_tool_result(&self) -> Option<&ClientToolResult> {
        match self {
            Self::ClientToolResult(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_client_tool_result(self) -> Option<ClientToolResult> {
        match self {
            Self::ClientToolResult(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_mcp_tool_approval_result(&self) -> bool {
        matches!(self, Self::McpToolApprovalResult(_))
    }

    pub fn as_mcp_tool_approval_result(&self) -> Option<&McpToolApprovalResult> {
        match self {
            Self::McpToolApprovalResult(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_mcp_tool_approval_result(self) -> Option<McpToolApprovalResult> {
        match self {
            Self::McpToolApprovalResult(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_contextual_update(&self) -> bool {
        matches!(self, Self::ContextualUpdate(_))
    }

    pub fn as_contextual_update(&self) -> Option<&ContextualUpdate> {
        match self {
            Self::ContextualUpdate(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_contextual_update(self) -> Option<ContextualUpdate> {
        match self {
            Self::ContextualUpdate(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_conversation_initiation_client_data(&self) -> bool {
        matches!(self, Self::ConversationInitiationClientData(_))
    }

    pub fn as_conversation_initiation_client_data(
        &self,
    ) -> Option<&ConversationInitiationClientData> {
        match self {
            Self::ConversationInitiationClientData(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_conversation_initiation_client_data(
        self,
    ) -> Option<ConversationInitiationClientData> {
        match self {
            Self::ConversationInitiationClientData(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for PublishEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserAudioChunk(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::Feedback(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::UserMessage(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::UserActivity(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::MultimodalMessage(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ConversationPong(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ClientToolResult(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::McpToolApprovalResult(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ContextualUpdate(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ConversationInitiationClientData(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for PublishEvent {
    fn default() -> Self {
        PublishEvent::UserMessage(Default::default())
    }
}
