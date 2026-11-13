pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ToolRequestModelToolConfig {
    #[serde(rename = "client")]
    Client {
        #[serde(flatten)]
        data: ClientToolConfigInput,
    },

    #[serde(rename = "mcp")]
    Mcp {
        #[serde(flatten)]
        data: McpToolConfigInput,
    },

    #[serde(rename = "system")]
    System {
        #[serde(flatten)]
        data: SystemToolConfigInput,
    },

    #[serde(rename = "webhook")]
    Webhook {
        #[serde(flatten)]
        data: WebhookToolConfigInput,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ToolRequestModelToolConfig {
    pub fn is_client(&self) -> bool {
        matches!(self, Self::Client { .. })
    }

    pub fn as_client(&self) -> Option<&ClientToolConfigInput> {
        match self {
            Self::Client { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_client(self) -> Option<ClientToolConfigInput> {
        match self {
            Self::Client { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_mcp(&self) -> bool {
        matches!(self, Self::Mcp { .. })
    }

    pub fn as_mcp(&self) -> Option<&McpToolConfigInput> {
        match self {
            Self::Mcp { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_mcp(self) -> Option<McpToolConfigInput> {
        match self {
            Self::Mcp { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_system(&self) -> bool {
        matches!(self, Self::System { .. })
    }

    pub fn as_system(&self) -> Option<&SystemToolConfigInput> {
        match self {
            Self::System { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_system(self) -> Option<SystemToolConfigInput> {
        match self {
            Self::System { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_webhook(&self) -> bool {
        matches!(self, Self::Webhook { .. })
    }

    pub fn as_webhook(&self) -> Option<&WebhookToolConfigInput> {
        match self {
            Self::Webhook { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_webhook(self) -> Option<WebhookToolConfigInput> {
        match self {
            Self::Webhook { data } => Some(data),
            _ => None,
        }
    }
}
