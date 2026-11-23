pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum SubscribeEvent {
    #[serde(rename = "conversation_initiation_metadata")]
    ConversationInitiationMetadata {
        #[serde(flatten)]
        data: ConversationInitiationMetadata,
    },

    #[serde(rename = "agent_response_complete")]
    AgentResponseComplete {
        #[serde(flatten)]
        data: AgentResponseComplete,
    },

    #[serde(rename = "context_usage")]
    ContextUsage {
        #[serde(flatten)]
        data: ContextUsage,
    },

    #[serde(rename = "mcp_tool_call")]
    McpToolCall {
        #[serde(flatten)]
        data: McpToolCall,
    },

    #[serde(rename = "client_error")]
    ClientError {
        #[serde(flatten)]
        data: ClientError,
    },

    #[serde(rename = "guardrail_triggered")]
    GuardrailTriggered {
        #[serde(flatten)]
        data: GuardrailTriggered,
    },

    #[serde(rename = "user_transcript")]
    UserTranscript {
        #[serde(flatten)]
        data: ConversationUserTranscript,
    },

    #[serde(rename = "agent_response")]
    AgentResponse {
        #[serde(flatten)]
        data: ConversationAgentResponse,
    },

    #[serde(rename = "queue_status")]
    QueueStatus {
        #[serde(flatten)]
        data: QueueStatus,
    },

    #[serde(rename = "agent_response_correction")]
    AgentResponseCorrection {
        #[serde(flatten)]
        data: AgentResponseCorrection,
    },

    #[serde(rename = "agent_response_metadata")]
    AgentResponseMetadata {
        #[serde(flatten)]
        data: AgentResponseMetadata,
    },

    #[serde(rename = "audio")]
    Audio {
        #[serde(flatten)]
        data: AudioResponse,
    },

    #[serde(rename = "interruption")]
    Interruption {
        #[serde(flatten)]
        data: Interruption,
    },

    #[serde(rename = "vad_score")]
    VadScore {
        #[serde(flatten)]
        data: VadScore,
    },

    #[serde(rename = "agent_chat_response_part")]
    AgentChatResponsePart {
        #[serde(flatten)]
        data: AgentChatResponsePart,
    },

    #[serde(rename = "client_tool_call")]
    ClientToolCall {
        #[serde(flatten)]
        data: ClientToolCall,
    },

    #[serde(rename = "agent_tool_response")]
    AgentToolResponse {
        #[serde(flatten)]
        data: AgentToolResponse,
    },

    #[serde(rename = "agent_tool_response_full_payload")]
    AgentToolResponseFullPayload {
        #[serde(flatten)]
        data: AgentToolResponseFullPayload,
    },

    #[serde(rename = "agent_tool_request")]
    AgentToolRequest {
        #[serde(flatten)]
        data: AgentToolRequest,
    },

    #[serde(rename = "ping")]
    Ping {
        #[serde(flatten)]
        data: ConversationPing,
    },

    #[serde(rename = "mcp_connection_status")]
    McpConnectionStatus {
        #[serde(flatten)]
        data: McpConnectionStatus,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl SubscribeEvent {
    pub fn is_conversation_initiation_metadata(&self) -> bool {
        matches!(self, Self::ConversationInitiationMetadata { .. })
    }

    pub fn as_conversation_initiation_metadata(&self) -> Option<&ConversationInitiationMetadata> {
        match self {
            Self::ConversationInitiationMetadata { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_conversation_initiation_metadata(self) -> Option<ConversationInitiationMetadata> {
        match self {
            Self::ConversationInitiationMetadata { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_agent_response_complete(&self) -> bool {
        matches!(self, Self::AgentResponseComplete { .. })
    }

    pub fn as_agent_response_complete(&self) -> Option<&AgentResponseComplete> {
        match self {
            Self::AgentResponseComplete { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_agent_response_complete(self) -> Option<AgentResponseComplete> {
        match self {
            Self::AgentResponseComplete { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_context_usage(&self) -> bool {
        matches!(self, Self::ContextUsage { .. })
    }

    pub fn as_context_usage(&self) -> Option<&ContextUsage> {
        match self {
            Self::ContextUsage { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_context_usage(self) -> Option<ContextUsage> {
        match self {
            Self::ContextUsage { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_mcp_tool_call(&self) -> bool {
        matches!(self, Self::McpToolCall { .. })
    }

    pub fn as_mcp_tool_call(&self) -> Option<&McpToolCall> {
        match self {
            Self::McpToolCall { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_mcp_tool_call(self) -> Option<McpToolCall> {
        match self {
            Self::McpToolCall { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_client_error(&self) -> bool {
        matches!(self, Self::ClientError { .. })
    }

    pub fn as_client_error(&self) -> Option<&ClientError> {
        match self {
            Self::ClientError { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_client_error(self) -> Option<ClientError> {
        match self {
            Self::ClientError { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_guardrail_triggered(&self) -> bool {
        matches!(self, Self::GuardrailTriggered { .. })
    }

    pub fn as_guardrail_triggered(&self) -> Option<&GuardrailTriggered> {
        match self {
            Self::GuardrailTriggered { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_guardrail_triggered(self) -> Option<GuardrailTriggered> {
        match self {
            Self::GuardrailTriggered { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_user_transcript(&self) -> bool {
        matches!(self, Self::UserTranscript { .. })
    }

    pub fn as_user_transcript(&self) -> Option<&ConversationUserTranscript> {
        match self {
            Self::UserTranscript { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_user_transcript(self) -> Option<ConversationUserTranscript> {
        match self {
            Self::UserTranscript { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_agent_response(&self) -> bool {
        matches!(self, Self::AgentResponse { .. })
    }

    pub fn as_agent_response(&self) -> Option<&ConversationAgentResponse> {
        match self {
            Self::AgentResponse { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_agent_response(self) -> Option<ConversationAgentResponse> {
        match self {
            Self::AgentResponse { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_queue_status(&self) -> bool {
        matches!(self, Self::QueueStatus { .. })
    }

    pub fn as_queue_status(&self) -> Option<&QueueStatus> {
        match self {
            Self::QueueStatus { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_queue_status(self) -> Option<QueueStatus> {
        match self {
            Self::QueueStatus { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_agent_response_correction(&self) -> bool {
        matches!(self, Self::AgentResponseCorrection { .. })
    }

    pub fn as_agent_response_correction(&self) -> Option<&AgentResponseCorrection> {
        match self {
            Self::AgentResponseCorrection { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_agent_response_correction(self) -> Option<AgentResponseCorrection> {
        match self {
            Self::AgentResponseCorrection { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_agent_response_metadata(&self) -> bool {
        matches!(self, Self::AgentResponseMetadata { .. })
    }

    pub fn as_agent_response_metadata(&self) -> Option<&AgentResponseMetadata> {
        match self {
            Self::AgentResponseMetadata { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_agent_response_metadata(self) -> Option<AgentResponseMetadata> {
        match self {
            Self::AgentResponseMetadata { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_audio(&self) -> bool {
        matches!(self, Self::Audio { .. })
    }

    pub fn as_audio(&self) -> Option<&AudioResponse> {
        match self {
            Self::Audio { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_audio(self) -> Option<AudioResponse> {
        match self {
            Self::Audio { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_interruption(&self) -> bool {
        matches!(self, Self::Interruption { .. })
    }

    pub fn as_interruption(&self) -> Option<&Interruption> {
        match self {
            Self::Interruption { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_interruption(self) -> Option<Interruption> {
        match self {
            Self::Interruption { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_vad_score(&self) -> bool {
        matches!(self, Self::VadScore { .. })
    }

    pub fn as_vad_score(&self) -> Option<&VadScore> {
        match self {
            Self::VadScore { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_vad_score(self) -> Option<VadScore> {
        match self {
            Self::VadScore { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_agent_chat_response_part(&self) -> bool {
        matches!(self, Self::AgentChatResponsePart { .. })
    }

    pub fn as_agent_chat_response_part(&self) -> Option<&AgentChatResponsePart> {
        match self {
            Self::AgentChatResponsePart { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_agent_chat_response_part(self) -> Option<AgentChatResponsePart> {
        match self {
            Self::AgentChatResponsePart { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_client_tool_call(&self) -> bool {
        matches!(self, Self::ClientToolCall { .. })
    }

    pub fn as_client_tool_call(&self) -> Option<&ClientToolCall> {
        match self {
            Self::ClientToolCall { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_client_tool_call(self) -> Option<ClientToolCall> {
        match self {
            Self::ClientToolCall { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_agent_tool_response(&self) -> bool {
        matches!(self, Self::AgentToolResponse { .. })
    }

    pub fn as_agent_tool_response(&self) -> Option<&AgentToolResponse> {
        match self {
            Self::AgentToolResponse { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_agent_tool_response(self) -> Option<AgentToolResponse> {
        match self {
            Self::AgentToolResponse { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_agent_tool_response_full_payload(&self) -> bool {
        matches!(self, Self::AgentToolResponseFullPayload { .. })
    }

    pub fn as_agent_tool_response_full_payload(&self) -> Option<&AgentToolResponseFullPayload> {
        match self {
            Self::AgentToolResponseFullPayload { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_agent_tool_response_full_payload(self) -> Option<AgentToolResponseFullPayload> {
        match self {
            Self::AgentToolResponseFullPayload { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_agent_tool_request(&self) -> bool {
        matches!(self, Self::AgentToolRequest { .. })
    }

    pub fn as_agent_tool_request(&self) -> Option<&AgentToolRequest> {
        match self {
            Self::AgentToolRequest { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_agent_tool_request(self) -> Option<AgentToolRequest> {
        match self {
            Self::AgentToolRequest { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_ping(&self) -> bool {
        matches!(self, Self::Ping { .. })
    }

    pub fn as_ping(&self) -> Option<&ConversationPing> {
        match self {
            Self::Ping { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_ping(self) -> Option<ConversationPing> {
        match self {
            Self::Ping { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_mcp_connection_status(&self) -> bool {
        matches!(self, Self::McpConnectionStatus { .. })
    }

    pub fn as_mcp_connection_status(&self) -> Option<&McpConnectionStatus> {
        match self {
            Self::McpConnectionStatus { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_mcp_connection_status(self) -> Option<McpConnectionStatus> {
        match self {
            Self::McpConnectionStatus { data } => Some(data),
            _ => None,
        }
    }
}
