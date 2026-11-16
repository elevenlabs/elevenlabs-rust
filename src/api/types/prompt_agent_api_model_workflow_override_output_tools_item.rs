pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum PromptAgentApiModelWorkflowOverrideOutputToolsItem {
    #[serde(rename = "api_integration_webhook")]
    ApiIntegrationWebhook {
        #[serde(flatten)]
        data: ApiIntegrationWebhookToolConfigOutput,
    },

    #[serde(rename = "client")]
    Client {
        #[serde(flatten)]
        data: ClientToolConfigOutput,
    },

    #[serde(rename = "mcp")]
    Mcp {
        #[serde(flatten)]
        data: McpToolConfigOutput,
    },

    #[serde(rename = "smb")]
    Smb {
        #[serde(flatten)]
        data: SmbToolConfig,
    },

    #[serde(rename = "system")]
    System {
        #[serde(flatten)]
        data: SystemToolConfigOutput,
    },

    #[serde(rename = "webhook")]
    Webhook {
        #[serde(flatten)]
        data: WebhookToolConfigOutput,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl PromptAgentApiModelWorkflowOverrideOutputToolsItem {
    pub fn is_api_integration_webhook(&self) -> bool {
        matches!(self, Self::ApiIntegrationWebhook { .. })
    }

    pub fn as_api_integration_webhook(&self) -> Option<&ApiIntegrationWebhookToolConfigOutput> {
        match self {
            Self::ApiIntegrationWebhook { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_api_integration_webhook(self) -> Option<ApiIntegrationWebhookToolConfigOutput> {
        match self {
            Self::ApiIntegrationWebhook { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_client(&self) -> bool {
        matches!(self, Self::Client { .. })
    }

    pub fn as_client(&self) -> Option<&ClientToolConfigOutput> {
        match self {
            Self::Client { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_client(self) -> Option<ClientToolConfigOutput> {
        match self {
            Self::Client { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_mcp(&self) -> bool {
        matches!(self, Self::Mcp { .. })
    }

    pub fn as_mcp(&self) -> Option<&McpToolConfigOutput> {
        match self {
            Self::Mcp { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_mcp(self) -> Option<McpToolConfigOutput> {
        match self {
            Self::Mcp { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_smb(&self) -> bool {
        matches!(self, Self::Smb { .. })
    }

    pub fn as_smb(&self) -> Option<&SmbToolConfig> {
        match self {
            Self::Smb { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_smb(self) -> Option<SmbToolConfig> {
        match self {
            Self::Smb { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_system(&self) -> bool {
        matches!(self, Self::System { .. })
    }

    pub fn as_system(&self) -> Option<&SystemToolConfigOutput> {
        match self {
            Self::System { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_system(self) -> Option<SystemToolConfigOutput> {
        match self {
            Self::System { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_webhook(&self) -> bool {
        matches!(self, Self::Webhook { .. })
    }

    pub fn as_webhook(&self) -> Option<&WebhookToolConfigOutput> {
        match self {
            Self::Webhook { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_webhook(self) -> Option<WebhookToolConfigOutput> {
        match self {
            Self::Webhook { data } => Some(data),
            _ => None,
        }
    }
}
