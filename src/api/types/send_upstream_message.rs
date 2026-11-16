pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum SendUpstreamMessage {
    #[serde(rename = "agent_response")]
    AgentResponse {
        #[serde(flatten)]
        data: AgentResponse,
    },

    #[serde(rename = "pong")]
    Pong {
        #[serde(flatten)]
        data: Pong,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl SendUpstreamMessage {
    pub fn is_agent_response(&self) -> bool {
        matches!(self, Self::AgentResponse { .. })
    }

    pub fn as_agent_response(&self) -> Option<&AgentResponse> {
        match self {
            Self::AgentResponse { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_agent_response(self) -> Option<AgentResponse> {
        match self {
            Self::AgentResponse { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_pong(&self) -> bool {
        matches!(self, Self::Pong { .. })
    }

    pub fn as_pong(&self) -> Option<&Pong> {
        match self {
            Self::Pong { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_pong(self) -> Option<Pong> {
        match self {
            Self::Pong { data } => Some(data),
            _ => None,
        }
    }
}
