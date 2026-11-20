pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum SendConversationMessageRequestInputItem {
    #[serde(rename = "client_tool_result")]
    ClientToolResult {
        #[serde(flatten)]
        data: ClientToolResultItem,
    },

    #[serde(rename = "contextual_update")]
    ContextualUpdate {
        #[serde(flatten)]
        data: ContextualUpdateInputItem,
    },

    #[serde(rename = "user_message")]
    UserMessage {
        #[serde(flatten)]
        data: UserMessageInputItem,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl SendConversationMessageRequestInputItem {
    pub fn is_client_tool_result(&self) -> bool {
        matches!(self, Self::ClientToolResult { .. })
    }

    pub fn as_client_tool_result(&self) -> Option<&ClientToolResultItem> {
        match self {
            Self::ClientToolResult { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_client_tool_result(self) -> Option<ClientToolResultItem> {
        match self {
            Self::ClientToolResult { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_contextual_update(&self) -> bool {
        matches!(self, Self::ContextualUpdate { .. })
    }

    pub fn as_contextual_update(&self) -> Option<&ContextualUpdateInputItem> {
        match self {
            Self::ContextualUpdate { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_contextual_update(self) -> Option<ContextualUpdateInputItem> {
        match self {
            Self::ContextualUpdate { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_user_message(&self) -> bool {
        matches!(self, Self::UserMessage { .. })
    }

    pub fn as_user_message(&self) -> Option<&UserMessageInputItem> {
        match self {
            Self::UserMessage { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_user_message(self) -> Option<UserMessageInputItem> {
        match self {
            Self::UserMessage { data } => Some(data),
            _ => None,
        }
    }
}
