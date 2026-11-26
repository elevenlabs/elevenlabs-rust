pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum PromptAgentApiModelWorkflowOverrideOutputToolsItem {
    WebhookToolConfigOutput(WebhookToolConfigOutput),
    ClientToolConfigOutput(ClientToolConfigOutput),
    SystemToolConfigOutput(SystemToolConfigOutput),
    McpToolConfigOutput(McpToolConfigOutput),
    ApiIntegrationWebhookToolConfigOutput(ApiIntegrationWebhookToolConfigOutput),
    SmbToolConfig(SmbToolConfig),

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    __Unknown(serde_json::Value),
}

impl<'de> serde::Deserialize<'de> for PromptAgentApiModelWorkflowOverrideOutputToolsItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;

        let mut matched_literal = false;

        if value.get("type").and_then(serde_json::Value::as_str) == Some("webhook") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<WebhookToolConfigOutput>(value.clone()) {
                return Ok(Self::WebhookToolConfigOutput(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("client") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<ClientToolConfigOutput>(value.clone()) {
                return Ok(Self::ClientToolConfigOutput(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("system") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<SystemToolConfigOutput>(value.clone()) {
                return Ok(Self::SystemToolConfigOutput(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("mcp") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<McpToolConfigOutput>(value.clone()) {
                return Ok(Self::McpToolConfigOutput(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("api_integration_webhook")
        {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<ApiIntegrationWebhookToolConfigOutput>(value.clone())
            {
                return Ok(Self::ApiIntegrationWebhookToolConfigOutput(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("smb") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<SmbToolConfig>(value.clone()) {
                return Ok(Self::SmbToolConfig(parsed));
            }
        }

        if !matched_literal {
            if value.get("type").is_some_and(serde_json::Value::is_string) {
                return Ok(Self::__Unknown(value));
            }

            if let Ok(parsed) = serde_json::from_value::<WebhookToolConfigOutput>(value.clone()) {
                return Ok(Self::WebhookToolConfigOutput(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<ClientToolConfigOutput>(value.clone()) {
                return Ok(Self::ClientToolConfigOutput(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<SystemToolConfigOutput>(value.clone()) {
                return Ok(Self::SystemToolConfigOutput(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<McpToolConfigOutput>(value.clone()) {
                return Ok(Self::McpToolConfigOutput(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<ApiIntegrationWebhookToolConfigOutput>(value.clone())
            {
                return Ok(Self::ApiIntegrationWebhookToolConfigOutput(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<SmbToolConfig>(value.clone()) {
                return Ok(Self::SmbToolConfig(parsed));
            }

            return Ok(Self::__Unknown(value));
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum PromptAgentApiModelWorkflowOverrideOutputToolsItem",
        ))
    }
}

impl PromptAgentApiModelWorkflowOverrideOutputToolsItem {
    pub fn is_webhook_tool_config_output(&self) -> bool {
        matches!(self, Self::WebhookToolConfigOutput(_))
    }

    pub fn as_webhook_tool_config_output(&self) -> Option<&WebhookToolConfigOutput> {
        match self {
            Self::WebhookToolConfigOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_webhook_tool_config_output(self) -> Option<WebhookToolConfigOutput> {
        match self {
            Self::WebhookToolConfigOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_client_tool_config_output(&self) -> bool {
        matches!(self, Self::ClientToolConfigOutput(_))
    }

    pub fn as_client_tool_config_output(&self) -> Option<&ClientToolConfigOutput> {
        match self {
            Self::ClientToolConfigOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_client_tool_config_output(self) -> Option<ClientToolConfigOutput> {
        match self {
            Self::ClientToolConfigOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_system_tool_config_output(&self) -> bool {
        matches!(self, Self::SystemToolConfigOutput(_))
    }

    pub fn as_system_tool_config_output(&self) -> Option<&SystemToolConfigOutput> {
        match self {
            Self::SystemToolConfigOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_system_tool_config_output(self) -> Option<SystemToolConfigOutput> {
        match self {
            Self::SystemToolConfigOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_mcp_tool_config_output(&self) -> bool {
        matches!(self, Self::McpToolConfigOutput(_))
    }

    pub fn as_mcp_tool_config_output(&self) -> Option<&McpToolConfigOutput> {
        match self {
            Self::McpToolConfigOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_mcp_tool_config_output(self) -> Option<McpToolConfigOutput> {
        match self {
            Self::McpToolConfigOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_api_integration_webhook_tool_config_output(&self) -> bool {
        matches!(self, Self::ApiIntegrationWebhookToolConfigOutput(_))
    }

    pub fn as_api_integration_webhook_tool_config_output(
        &self,
    ) -> Option<&ApiIntegrationWebhookToolConfigOutput> {
        match self {
            Self::ApiIntegrationWebhookToolConfigOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_api_integration_webhook_tool_config_output(
        self,
    ) -> Option<ApiIntegrationWebhookToolConfigOutput> {
        match self {
            Self::ApiIntegrationWebhookToolConfigOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_smb_tool_config(&self) -> bool {
        matches!(self, Self::SmbToolConfig(_))
    }

    pub fn as_smb_tool_config(&self) -> Option<&SmbToolConfig> {
        match self {
            Self::SmbToolConfig(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_smb_tool_config(self) -> Option<SmbToolConfig> {
        match self {
            Self::SmbToolConfig(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::__Unknown(_))
    }

    pub fn as_unknown(&self) -> Option<&serde_json::Value> {
        match self {
            Self::__Unknown(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_unknown(self) -> Option<serde_json::Value> {
        match self {
            Self::__Unknown(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for PromptAgentApiModelWorkflowOverrideOutputToolsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WebhookToolConfigOutput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ClientToolConfigOutput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::SystemToolConfigOutput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::McpToolConfigOutput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ApiIntegrationWebhookToolConfigOutput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::SmbToolConfig(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::__Unknown(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
