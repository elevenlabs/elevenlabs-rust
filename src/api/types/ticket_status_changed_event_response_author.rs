pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum TicketStatusChangedEventResponseAuthor {
    #[serde(rename = "ai_agent")]
    AiAgent {
        #[serde(flatten)]
        data: TicketEventAiAgentAuthorResponse,
    },

    #[serde(rename = "human_agent")]
    HumanAgent {
        #[serde(flatten)]
        data: TicketEventHumanAgentAuthorResponse,
    },

    #[serde(rename = "requester")]
    Requester {
        #[serde(flatten)]
        data: TicketEventRequesterAuthorResponse,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl TicketStatusChangedEventResponseAuthor {
    pub fn is_ai_agent(&self) -> bool {
        matches!(self, Self::AiAgent { .. })
    }

    pub fn as_ai_agent(&self) -> Option<&TicketEventAiAgentAuthorResponse> {
        match self {
            Self::AiAgent { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_ai_agent(self) -> Option<TicketEventAiAgentAuthorResponse> {
        match self {
            Self::AiAgent { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_human_agent(&self) -> bool {
        matches!(self, Self::HumanAgent { .. })
    }

    pub fn as_human_agent(&self) -> Option<&TicketEventHumanAgentAuthorResponse> {
        match self {
            Self::HumanAgent { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_human_agent(self) -> Option<TicketEventHumanAgentAuthorResponse> {
        match self {
            Self::HumanAgent { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_requester(&self) -> bool {
        matches!(self, Self::Requester { .. })
    }

    pub fn as_requester(&self) -> Option<&TicketEventRequesterAuthorResponse> {
        match self {
            Self::Requester { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_requester(self) -> Option<TicketEventRequesterAuthorResponse> {
        match self {
            Self::Requester { data } => Some(data),
            _ => None,
        }
    }
}
