pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Event {
    #[serde(rename = "agent_chat_response_part")]
    AgentChatResponsePart {
        #[serde(flatten)]
        data: AgentChatResponsePartClientEvent,
    },

    #[serde(rename = "agent_reasoning_response_part")]
    AgentReasoningResponsePart {
        #[serde(flatten)]
        data: AgentReasoningResponsePartClientEvent,
    },

    #[serde(rename = "agent_response")]
    AgentResponse {
        #[serde(flatten)]
        data: AgentResponseClientEvent,
    },

    #[serde(rename = "agent_tool_request")]
    AgentToolRequest {
        #[serde(flatten)]
        data: AgentToolRequestStreamedEvent,
    },

    #[serde(rename = "agent_tool_response")]
    AgentToolResponse {
        #[serde(flatten)]
        data: AgentToolResponseStreamedEvent,
    },

    #[serde(rename = "client_tool_call")]
    ClientToolCall {
        #[serde(flatten)]
        data: ClientToolCallEvent,
    },

    #[serde(rename = "stream_ended")]
    StreamEnded {
        #[serde(flatten)]
        data: StreamEndedEvent,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl Event {
    pub fn is_agent_chat_response_part(&self) -> bool {
        matches!(self, Self::AgentChatResponsePart { .. })
    }

    pub fn as_agent_chat_response_part(&self) -> Option<&AgentChatResponsePartClientEvent> {
        match self {
            Self::AgentChatResponsePart { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_agent_chat_response_part(self) -> Option<AgentChatResponsePartClientEvent> {
        match self {
            Self::AgentChatResponsePart { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_agent_reasoning_response_part(&self) -> bool {
        matches!(self, Self::AgentReasoningResponsePart { .. })
    }

    pub fn as_agent_reasoning_response_part(
        &self,
    ) -> Option<&AgentReasoningResponsePartClientEvent> {
        match self {
            Self::AgentReasoningResponsePart { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_agent_reasoning_response_part(
        self,
    ) -> Option<AgentReasoningResponsePartClientEvent> {
        match self {
            Self::AgentReasoningResponsePart { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_agent_response(&self) -> bool {
        matches!(self, Self::AgentResponse { .. })
    }

    pub fn as_agent_response(&self) -> Option<&AgentResponseClientEvent> {
        match self {
            Self::AgentResponse { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_agent_response(self) -> Option<AgentResponseClientEvent> {
        match self {
            Self::AgentResponse { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_agent_tool_request(&self) -> bool {
        matches!(self, Self::AgentToolRequest { .. })
    }

    pub fn as_agent_tool_request(&self) -> Option<&AgentToolRequestStreamedEvent> {
        match self {
            Self::AgentToolRequest { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_agent_tool_request(self) -> Option<AgentToolRequestStreamedEvent> {
        match self {
            Self::AgentToolRequest { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_agent_tool_response(&self) -> bool {
        matches!(self, Self::AgentToolResponse { .. })
    }

    pub fn as_agent_tool_response(&self) -> Option<&AgentToolResponseStreamedEvent> {
        match self {
            Self::AgentToolResponse { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_agent_tool_response(self) -> Option<AgentToolResponseStreamedEvent> {
        match self {
            Self::AgentToolResponse { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_client_tool_call(&self) -> bool {
        matches!(self, Self::ClientToolCall { .. })
    }

    pub fn as_client_tool_call(&self) -> Option<&ClientToolCallEvent> {
        match self {
            Self::ClientToolCall { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_client_tool_call(self) -> Option<ClientToolCallEvent> {
        match self {
            Self::ClientToolCall { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_stream_ended(&self) -> bool {
        matches!(self, Self::StreamEnded { .. })
    }

    pub fn as_stream_ended(&self) -> Option<&StreamEndedEvent> {
        match self {
            Self::StreamEnded { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_stream_ended(self) -> Option<StreamEndedEvent> {
        match self {
            Self::StreamEnded { data } => Some(data),
            _ => None,
        }
    }
}
