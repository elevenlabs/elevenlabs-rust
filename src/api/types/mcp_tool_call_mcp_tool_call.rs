pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "state")]
#[non_exhaustive]
pub enum McpToolCallMcpToolCall {
    #[serde(rename = "loading")]
    Loading {
        #[serde(flatten)]
        data: Loading,
    },

    #[serde(rename = "awaiting_approval")]
    AwaitingApproval {
        #[serde(flatten)]
        data: AwaitingApproval,
    },

    #[serde(rename = "success")]
    Success {
        #[serde(flatten)]
        data: Success,
    },

    #[serde(rename = "failure")]
    Failure {
        #[serde(flatten)]
        data: Failure,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl McpToolCallMcpToolCall {
    pub fn is_loading(&self) -> bool {
        matches!(self, Self::Loading { .. })
    }

    pub fn as_loading(&self) -> Option<&Loading> {
        match self {
            Self::Loading { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_loading(self) -> Option<Loading> {
        match self {
            Self::Loading { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_awaiting_approval(&self) -> bool {
        matches!(self, Self::AwaitingApproval { .. })
    }

    pub fn as_awaiting_approval(&self) -> Option<&AwaitingApproval> {
        match self {
            Self::AwaitingApproval { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_awaiting_approval(self) -> Option<AwaitingApproval> {
        match self {
            Self::AwaitingApproval { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success { .. })
    }

    pub fn as_success(&self) -> Option<&Success> {
        match self {
            Self::Success { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_success(self) -> Option<Success> {
        match self {
            Self::Success { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_failure(&self) -> bool {
        matches!(self, Self::Failure { .. })
    }

    pub fn as_failure(&self) -> Option<&Failure> {
        match self {
            Self::Failure { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_failure(self) -> Option<Failure> {
        match self {
            Self::Failure { data } => Some(data),
            _ => None,
        }
    }
}
