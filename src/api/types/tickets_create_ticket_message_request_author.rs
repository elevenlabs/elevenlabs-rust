pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CreateTicketMessageRequestAuthor {
    #[serde(rename = "human_agent")]
    HumanAgent {
        #[serde(flatten)]
        data: TicketHumanAgentAuthorRequest,
    },

    #[serde(rename = "requester")]
    Requester {
        #[serde(flatten)]
        data: TicketRequesterAuthorRequest,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl CreateTicketMessageRequestAuthor {
    pub fn is_human_agent(&self) -> bool {
        matches!(self, Self::HumanAgent { .. })
    }

    pub fn as_human_agent(&self) -> Option<&TicketHumanAgentAuthorRequest> {
        match self {
            Self::HumanAgent { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_human_agent(self) -> Option<TicketHumanAgentAuthorRequest> {
        match self {
            Self::HumanAgent { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_requester(&self) -> bool {
        matches!(self, Self::Requester { .. })
    }

    pub fn as_requester(&self) -> Option<&TicketRequesterAuthorRequest> {
        match self {
            Self::Requester { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_requester(self) -> Option<TicketRequesterAuthorRequest> {
        match self {
            Self::Requester { data } => Some(data),
            _ => None,
        }
    }
}
