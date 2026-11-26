pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Requester {
    #[serde(rename = "email")]
    Email {
        #[serde(flatten)]
        data: TicketRequesterByEmailRequest,
    },

    #[serde(rename = "slack")]
    Slack {
        #[serde(flatten)]
        data: TicketRequesterBySlackRequest,
    },

    #[serde(rename = "user_id")]
    UserId {
        #[serde(flatten)]
        data: TicketRequesterByUserIdRequest,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl Requester {
    pub fn is_email(&self) -> bool {
        matches!(self, Self::Email { .. })
    }

    pub fn as_email(&self) -> Option<&TicketRequesterByEmailRequest> {
        match self {
            Self::Email { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_email(self) -> Option<TicketRequesterByEmailRequest> {
        match self {
            Self::Email { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_slack(&self) -> bool {
        matches!(self, Self::Slack { .. })
    }

    pub fn as_slack(&self) -> Option<&TicketRequesterBySlackRequest> {
        match self {
            Self::Slack { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_slack(self) -> Option<TicketRequesterBySlackRequest> {
        match self {
            Self::Slack { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_user_id(&self) -> bool {
        matches!(self, Self::UserId { .. })
    }

    pub fn as_user_id(&self) -> Option<&TicketRequesterByUserIdRequest> {
        match self {
            Self::UserId { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_user_id(self) -> Option<TicketRequesterByUserIdRequest> {
        match self {
            Self::UserId { data } => Some(data),
            _ => None,
        }
    }
}
